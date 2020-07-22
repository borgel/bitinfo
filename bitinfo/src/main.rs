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
use log::{info, trace};
use std::fmt;

use serde::{Serialize, Deserialize};

const SEPARATORS: &str = ":./";

const CONFIG_FILE_NAME: &str = ".bitinfo.yaml";

// available printing preferences
#[derive(Debug, Clone, Copy)]
enum PrintPreference {
   // TODO bool?
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
impl From<Option<&String>> for PrintPreference {
   fn from(s: Option<&String>) -> Self {
      if let None = s {
         PrintPreference::Hex
      }
      else {
         PrintPreference::from(s.unwrap().to_lowercase().as_ref())
      }
   }
}
impl PrintPreference {
   fn format_val(&self, val: u32) -> String {
      match self {
         PrintPreference::Bin => format!("0b{:b}", val),
         PrintPreference::Hex => format!("0x{:X}", val),
         PrintPreference::Decimal => format!("{}", val),
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
   // print this in bin, dec, hex
   preferred_format: Option<String>,

   // bit patterns (in binary) which have special meanings for this mask. If this is unspecified
   // the tool will print it in the preferred format. If these are specified the value description
   // will be printed along with the value in this range (in the preferred format) if the
   // binary key is matched
   patterns: Option<HashMap<String, String>>,
}

// useful for passing around the final baked descriptions to format out to the user
#[derive(Debug)]
struct RegisterDescription {
   name: String,
   value: String,
   description: Option<String>,

   // to sort the order in which these should be printed
   sort: u32,
}
impl fmt::Display for RegisterDescription {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      let r = write!(f, "\t{} =\t{}", self.name, self.value);
      if let Some(d) = &self.description {
         write!(f, " ({})", d)
      }
      else {
         r
      }
   }
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
   fn format_value(self, val: u32) -> RegisterDescription {
      // shift down user value so we can use it more directly
      // TODO don't lock to u32
      let extracted_mask: u32 = self.bitmask[..].load::<u32>();
      let val_masked = (val & extracted_mask) >> self.base_offset;

      let decoded_value = match self.patterns.get(&val_masked) {
         Some(v) => {
            format!("{}", v)
         }
         _ => {
            self.print_format.format_val(val_masked)
         }
      };

      RegisterDescription {
         name: self.name.clone(),
         value: decoded_value,
         description: self.description,
         sort: self.base_offset,
      }
   }
}
// TODO implement fmt so we can print an inflated mask?

fn main() {
   env_logger::init();
   let app = App::new("A tool to tell you about the bits in your registers")
      .setting(AppSettings::TrailingVarArg)
      .arg(Arg::with_name("bits")
           .long("bits")
           .help("Print which i'th bits are set for numbers with no other available formatters")
           .required(false)
           .takes_value(false))
      .arg(Arg::with_name("inputs")
           .multiple(true)
           .help("Values to display and format")
           .takes_value(true)
          );
   let options = app.get_matches();

   trace!("{:#?}", options);

   // get the normal args
   let print_each_bit = options.is_present("bits");

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
   trace!("Loaded {} configs", configs.len());

   let mut results: Option<(RegisterDescription, Vec<RegisterDescription>)> = None;
   for td in to_decode {

      // split on anything that isn't a number
      let mut sp: Vec<&str> = td.split(|c: char| SEPARATORS.contains(c)).collect();
      if sp.len() > 1 {
         // get the last segment of the vec which should be the final value
         let numeric_val = sp.pop().unwrap();
         if let Ok(nv) = parse::<u32>(numeric_val) {
            results = Some(smart_decode(nv, sp, &configs));
         }
      }
      else {
         if let Ok(as_integer) = parse::<u32>(td) {
            results = Some(print_bits(as_integer, print_each_bit));
         }
      }

      // print all the info for this value
      if let Some(final_results) = &results {
         println!("{}", final_results.0);
         for f in &final_results.1 {
            println!("  {}", &f);
         }
      }
   }
}

fn smart_decode(number: u32, keys: Vec<&str>, configs: &InfoMap) -> (RegisterDescription, Vec<RegisterDescription>) {
   trace!("\n{:?}: {:?}", keys, number);

   let name = keys.last().unwrap().clone();

   let decoder = match find_config_for_name(&keys, configs) {
      Some(d) => d,
      None => {
         // if we don't have a config, just print the bits
         return print_bits(number, false);
      }
   };

   info!("found a decoder for {}! {:?}", &name, &decoder);

   // prep the list of decoders for this BitInfo's fields
   let decoders = prep_decoders(&decoder);

   info!("inflated deocders to this list:\n{:#?}", decoders);

   // print the value of all decoders, they will mask out the relevant sections of the user's
   // value. Anything not in a given pattern is considered 'reserved' and not printed
   let mut all_formats: Vec<RegisterDescription> = Vec::new();
   for d in decoders {
      all_formats.push(d.format_value(number));
   }

   all_formats.sort_by(|a, b| a.sort.cmp(&b.sort));

   // final user output
   let default_format = PrintPreference::from(decoder.preferred_format.as_ref());
   (
      RegisterDescription {
         name: format!("{} {} ->", keys.join("."), default_format.format_val(number)),
            value: "".to_string(),
            description: None,
            sort: 0,
      },
      all_formats
   )
}

fn find_config_for_name<'a>(keys: &Vec<&str>, config: &'a InfoMap) -> Option<&'a BitInfo> {
   // use .get so it returns an Option instead of a panic
   // TODO make hashmap search case insensitive, which is surprisingly hard
   let mut local_keys = keys.clone();
   if let Some(cfg) = config.get(local_keys[0]) {
      local_keys.remove(0);

      // if the BitInfo has more 'registers' than recurse, otherwise we are at the end
      if let Some(r) = cfg.registers.as_ref() {
         return find_config_for_name(&local_keys, r);
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

fn print_bits(number: u32, print_each_bit: bool) -> (RegisterDescription, Vec<RegisterDescription>) {
   let header = RegisterDescription {
      name: format!("{}", number),
      value: format!("{} {:#X} {:#b}", number, number, number),
      description: None,
      sort: 0,
   };

   let mut extra_info: Vec<RegisterDescription> = Vec::new();

   if print_each_bit {
      let bits = 8 * size_of_val(&number);
      let mut number_to_eat = number;
      for i in 0..bits {
         if (number_to_eat & 0x1) != 0 {
            extra_info.push(RegisterDescription {
               name: "".to_string(),
               value: format!("{}th set", i),
               description: None,
               sort: i as u32,
            });
         }
         number_to_eat >>= 1;

         if number_to_eat == 0 {
            break;
         }
      }
   }
   (header, extra_info)
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

   info!("Opened {:?}", &f);


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

   trace!("inflated info {:#?}\n\n", &inflated);
   Ok(inflated)
}

