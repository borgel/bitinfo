use clap::{Arg, App, AppSettings};
use parse_int::parse;
use std::mem::size_of_val;
use std::env;
use std::path::{PathBuf};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use bitvec::prelude::*;
use std::iter::repeat;

use serde::{Serialize, Deserialize};

const SEPARATORS: &str = ":./";

const CONFIG_FILE_NAME: &str = ".bitinfo.yaml";

// available printing preferences
#[derive(Debug, Clone, Copy)]
enum PrintPreference {
   Bin,
   Hex,
   Decimal,
}
impl From<&str> for PrintPreference {
   fn from(s: &str) -> Self {
      match s.to_lowercase().as_ref() {
         "bin" => PrintPreference::Bin,
         "binary" => PrintPreference::Bin,
         "hex" => PrintPreference::Hex,
         "dec" => PrintPreference::Decimal,
         "decimal" => PrintPreference::Decimal,
         // if we didn't match, assume the default
         _ => PrintPreference::Hex,
      }
   }
}

type InfoMap = HashMap<String, BitInfo>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BitInfo {
   // name is implicit as the 'key' to this value
   description: Option<String>,

   // optional so it can be determined by summing all the children
   bit_width: Option<u32>,
   // set the default print format for all children
   preferred_format: Option<String>,

   // TODO find a way to make these names less bad
   registers: Option<HashMap<String, BitInfo>>,
   fields: Option<HashMap<String, RegisterMask>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct RegisterMask{
   start: u32,
   // specify either end or width or both. If neither is specified the mask is assumed to be one
   // bit wide
   end: Option<u32>,
   width: Option<u32>,

   description: Option<String>,
   negated: Option<bool>,
   // print this in bin, dec, hex
   preferred_format: Option<String>,

   // bit patterns (in binary) which have special meanings for this mask. If this is unspecified
   // the tool will print it in the preferred format. If these are specified the value description
   // will be printed along with the value in this range (in the preferred format) if the
   // binary key is matched
   patterns: Option<HashMap<String, String>>,
}

// a RegisterMask which has been inflated and ready to use
#[derive(Debug)]
struct InflatedRegisterMask {
   name: String,
   description: Option<String>,
   print_format: PrintPreference,

   bitmask: BitVec,
   base_offset: u32,
   width: u32,

   patterns: HashMap<u32, String>,
}
impl InflatedRegisterMask {
   fn try_from(rm: &RegisterMask, name: &String, default_print_pref: &PrintPreference) -> Option<Self> {
      let width = match rm.width {
         Some(w) => w,
         None => match rm.end {
            Some(end) => end - rm.start,
            // if there is neither an end nor a width, assume the field is one bit wide
            None => 1,
         }
      };

      // reconstruct the actual mask from the description
      // local bit ordering
      let mut mask = bitvec![Local;];
      // leading 0s
      mask.extend(repeat(false).take(rm.start as usize));
      // add the 1s
      mask.extend(repeat(true).take(width as usize));

      // reformat the descriptive patterns
      let mut pattern_map: HashMap<u32, String> = HashMap::new();
      if let Some(pm) = &rm.patterns {
         for (bits, description) in pm {
            // we expect the bit patterns to be in some numeric format, which we can parse
            // directly to a scalar
            if let Ok(parsed) = parse::<u32>(&bits) {
               pattern_map.insert(parsed, description.to_string());
            }
         }
      }

      // figure out how to print this
      let mut print_pref = default_print_pref.clone();
      if let Some(pf) = &rm.preferred_format {
         print_pref = PrintPreference::from(pf.as_ref());
      }

      Some(InflatedRegisterMask {
         name: name.to_string(),
         description: rm.description.clone(),
         print_format: print_pref,
         bitmask: mask,
         base_offset: rm.start,
         width: width,
         patterns: pattern_map,
      })
   }
   fn format_value(self, val: u32) -> Vec<(String,String)> {
      let mut formats: Vec<(String, String)> = Vec::new();

      // shift down user value so we can use it more directly
      let extracted_mask: u32 = self.bitmask[..].load::<u32>();
      let val_masked = (val & extracted_mask) >> self.base_offset;

      if self.patterns.len() > 0 {
         if self.patterns.contains_key(&val_masked) {
            formats.push((format!("{}", self.name), self.patterns[&val_masked].clone()));
         }
      }
      else {
         // no patterns for this range, format it and return
         // TODO obey preferred format
         // TODO obey negated
         // TODO add desription (new struct for all three?)
         let mut t = (format!("{}", self.name), format!("{:X}", val_masked));
         if let Some(desc) = self.description {
            t = (format!("{}", self.name), format!("{:X} ({})", val_masked, desc));
         }
         formats.push(t);
      }
      formats
   }
}
// TODO implement fmt so we can print an inflated mask?

fn main() {
   let app = App::new("The bitinfo tool to tell you about the bits in your registers")
      .setting(AppSettings::TrailingVarArg)
      // TODO more args
      .arg(Arg::with_name("inputs")
           .multiple(true)
          );
   let options = app.get_matches();

   // check to see if there are any trailing args and get them
   let to_decode: Vec<&str>;
   if let Some(td) = options.values_of("inputs") {
      to_decode = td.collect();
   }
   else {
      return ();
   }

   // begin loading all .bitinfo files in PWD
   let configs = load_configs();
   println!("Loaded {} configs", configs.len());

   for td in to_decode {
      // split on anything that isn't a number
      let mut sp: Vec<&str> = td.split(|c: char| SEPARATORS.contains(c)).collect();
      if sp.len() > 1 {
         // get the last segment of the vec which should be the final value
         let numeric_val = sp.pop().unwrap();
         if let Ok(nv) = parse::<u32>(numeric_val) {
            smart_decode(nv, sp, &configs);
         }
         continue;
      }
      else {
         if let Ok(as_integer) = parse::<u32>(td) {
            print_bits(as_integer);
         }
      }
   }
}

fn smart_decode(number: u32, mut keys: Vec<&str>, configs: &InfoMap) {
   println!("\n{:?}: {:?}", keys, number);

   let name = keys.last().unwrap().clone();

   // TODO decode from .bitinfo map
   let decoder = match find_config_for_name(&mut keys, configs) {
      Some(d) => d,
      None => {
         // if we don't have a config, just print the bits
         print_bits(number);
         return
      }
   };

   // FIXME rm
   println!("found a decoder for {}! {:?}", &name, &decoder);

   // prep the list of decoders for this BitInfo's fields
   let decoders = prep_decoders(&decoder);

   // TODO use a logger to put behind info flags
   // FIXME rm
   println!("inflated deocders to this list:\n{:#?}", decoders);

   // print the value of all decoders, they will mask out the relevant sections of the user's
   // value. Anything not in a given pattern is considered 'reserved' and not printed
   // TODO sort keys by start bit
   let mut all_formats: Vec<(String, String)> = Vec::new();
   for d in decoders {
      all_formats.extend(d.format_value(number));
   }

   // final user output
   // TODO obey user format
   println!("0x{:X} ->", number);
   for f in all_formats {
      println!("\t{} =\t{}", f.0, f.1);
   }
}

fn find_config_for_name<'a>(keys: &mut Vec<&str>, config: &'a InfoMap) -> Option<&'a BitInfo> {
   // use .get so it returns an Option instead of a panic
   // TODO make hashmap search case insensitive, which is surprisingly hard
   if let Some(cfg) = config.get(keys[0]) {
      keys.remove(0);

      // if the BitInfo has more 'registers' than recurse, otherwise we are at the end
      if let Some(r) = cfg.registers.as_ref() {
         return find_config_for_name(keys, r);
      }
      else {
         return Some(cfg)
      }
   }
   return None
}

fn prep_decoders(raw_dec: &BitInfo) -> Vec<InflatedRegisterMask>  {
   let parent_format = match &raw_dec.preferred_format {
      Some(pf) => PrintPreference::from(pf.as_ref()),
      None => PrintPreference::Hex,
   };

   let mut decoders: Vec<InflatedRegisterMask> = Vec::new();

   let raw_fields = match &raw_dec.fields {
      Some(f) => f,
      None => return decoders,
   };

   for (reg_name, description) in raw_fields {
      if let Some(dc) = InflatedRegisterMask::try_from(&description, reg_name, &parent_format) {
         decoders.push(dc);
      }
   }

   return decoders
}

fn print_bits(number: u32) {
   println!("\"{}\" -> {} {:#X} {:#b}", number, number, number, number);

   let bits = 8 * size_of_val(&number);
   let mut number_to_eat = number;
   for i in 0..bits {
      if (number_to_eat & 0x1) != 0 {
         println!("{}th set", i);
      }
      number_to_eat >>= 1;

      if number_to_eat == 0 {
         break;
      }
   }
}

// returns a map of all the loaded user configs to search for decoding, even if it's empty
fn load_configs() -> InfoMap {
   let mut all_infos: HashMap<String, BitInfo> = HashMap::new();
   let mut full_path = env::current_dir().unwrap();
   // TODO dispatch these in parallel
   loop {
      if let Ok(c) = load_config(&full_path) {
         all_infos.extend(c.into_iter());
      }
      if !full_path.pop() {
         break;
      }
   }
   all_infos
}

fn load_config(path: &PathBuf) -> Result<InfoMap, ()>  {
   let mut path = PathBuf::from(path);
   path.push(CONFIG_FILE_NAME);

   let mut f = match File::open(path) {
      Err(_) => return Err(()),
      Ok(file) => file,
   };

   println!("Opened {:?}", &f);


   // FIXME rm
   let mut s = String::new();
   f.read_to_string(&mut s).unwrap();

   let inflated: HashMap<String, BitInfo> = match serde_yaml::from_str(&s) {
      Ok(inf) => inf,
      Err(e) => {
         eprintln!("failed to inflate: {}", e);
         return Err(())
      },
   };

   // FIXME rm
   println!("inflated info {:?}\n\n", &inflated);
   for (name, config) in &inflated {
      println!("One: {}:{:?}", name, config);
      let hm = config.registers.as_ref().unwrap();
      for r in hm {
         println!("  Two: {:?}", r);
      }
   }

   Ok(inflated)
}

// FIXME rm
/*
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
*/

