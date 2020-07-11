use clap::{Arg, App, AppSettings};
use parse_int::parse;
use std::mem::size_of_val;
use std::env;
use std::path::{PathBuf};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use yaml_rust::{YamlLoader, yaml};
use yaml_rust::Yaml::{Hash};

use serde::{Serialize, Deserialize};


// TODO support more separators
const SEPARATORS: &str = ":";

const CONFIG_FILE_NAME: &str = ".bitinfo.yaml";

const KEY_MASK: &str = "masks";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BitInfo {
   // name is implicit as the 'key' to this value
   description: Option<String>,

   fields: Option<HashMap<String, BitInfo>>,
   registers: Option<HashMap<String, BitRange>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BitRange {
   // name is implicit as the 'key' to this value
   description: Option<String>,

   // optional so it can be determined by summing all the children
   bit_width: Option<u32>,

   masks: Option<HashMap<String, BitMask>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct BitMask {
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
   load_configs();

   // FIXME rm
   return ();

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

// top level class name:info struct
fn load_configs() -> HashMap<String, BitInfo> {
   let mut all_infos: HashMap<String, BitInfo> = HashMap::new();
   let mut full_path = env::current_dir().unwrap();
   // TODO dispatch these in parallel
   loop {
      load_config(&full_path);
      /*
      if let Some(bi) = load_config(&full_path) {
         all_infos.extend(bi.into_iter());
      }
      */
      if !full_path.pop() {
         break;
      }
   }
   all_infos
}

fn load_config(path: &PathBuf) -> Result<BitInfo, ()>  {
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

   println!("inflated info {:?}", &inflated);

   Err(())

   /*
   let config = match YamlLoader::load_from_str(&s) {
      Err(_) => return None,
      Ok(cfg) => cfg,
   };

   // we expect this config to be an array of hashmaps
   rethink this. why flatten it? build datastructures (or maps of maps) to hold
      the info. should probably be structs with maps. think about what data needs to be held
      and orgamized how

   let mut flattened_maps = Bitranges::new();

   // FIXME rm
   println!("{:?}", config);

   for c in config {
      for hm in c {
         match hm {
            yaml_rust::Yaml::Hash(h) => {
               if let Some(flatmap) = flatten_hashmap(h) {
                  // collapse together
                  flattened_maps.extend(flatmap.into_iter());
               }
            },
            _ => ()
         };
      }
   }

   Some(flattened_maps)
   */
}

/*
/*
 * Flatten all the layers of nested hashmaps under this node and return
 * them as a single layer hashmap. So concatenate all the keys together,
 * to the same value.
 */
fn flatten_hashmap(yhash: &yaml::Hash) -> Option<Bitranges> {
   // recursively return key:value?
   println!("\n flatten: {:?}", yhash);
   // if contains 
   //println!("{:?}", yhash[KEY_MASK]);
   for (k, v) in yhash {
      println!("f {:?}:{:?}", k, v);
      //print_type_of(&k);
   }
   match yhash {
      yaml::Yaml::Array(ref v) => {
         for x in v {
            flatten_hashmap(x);
         }
      }
      yaml::Yaml::Hash(ref h) => {
         for (k, v) in h {
            println!("{:?}:", k);
            flatten_hashmap(v);
         }
      }
      _ => {
         println!("END {:?}", doc);
         //return format!("{:?}", doc);
         return doc;
      }
   }
   None
      /*
      yaml_rust::Yaml::Array(a) => {
         ()
      },
      yaml_rust::Yaml::Hash(h) => {
         println!("hash");
         //flatten_hashmap(h);

         // FIXME rm
         ()
      },
   }
   None
   */
}

// FIXME rm
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn find_config_for_name(bin_name: &str) {
   // search up PWD looking for .bitinfo  files
}
*/

