use clap::{Arg, App, AppSettings};
use parse_int::parse;
use std::mem::size_of_val;
use std::env;

// TODO support more separators
const SEPARATORS: &str = ":";

fn main() {
   let app = App::new("Trailing args example")
      .setting(AppSettings::TrailingVarArg)
      // TODO more args
      .arg(Arg::with_name("inputs")
           .multiple(true)
          );
   let options = app.get_matches();

   println!("{:?}", options);
   let to_decode: Vec<&str> = options.values_of("inputs").unwrap().collect();
   println!("{:?}", to_decode);

   // TODO begin loading all .bitinfo files in PWD

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

   // TODO decode from .bitinfo
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

fn find_config_for_name(bin_name: &str) {
   // search up PWD looking for .bitinfo  files
}

