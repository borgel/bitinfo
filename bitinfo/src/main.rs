use clap::{Arg, App, AppSettings};
use parse_int::parse;
use std::mem::size_of_val;
use std::env;
use std::path::{PathBuf};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde::{Serialize, Deserialize};


// TODO support more separators
const SEPARATORS: &str = ":";

const CONFIG_FILE_NAME: &str = ".bitinfo.yaml";

const KEY_MASK: &str = "masks";

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

// TODO second is a struct
//type Bitranges = HashMap<String, String>;

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
      if td.contains(SEPARATORS) {
         let mut sp: Vec<&str> = td.split(SEPARATORS).collect();
         let numeric_val = sp.pop().unwrap();
         if let Ok(nv) = parse::<u32>(numeric_val) {
            smart_decode(&sp.join(":"), nv);
         }
         continue;
      }
      if let Ok(as_integer) = parse::<u32>(td) {
         print_bits(td, as_integer);
      }
   }
}

fn smart_decode(key: &str, number: u32) {
   println!("\n{}: {:?}", key, number);

   // TODO decode from .bitinfo map
}

fn print_bits(raw_string: &str, number: u32) {
   println!("\"{}\" -> {} {:#X} {:#b}", raw_string, number, number, number);

   // see if we can find a config file for this bin

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
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn find_config_for_name(bin_name: &str) {
   // search up PWD looking for .bitinfo  files
}

