use clap::{Arg, App, AppSettings};
use parse_int::parse;
use std::mem::size_of_val;

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

   let mut decoded_one = false;
   for td in to_decode {
      if let Ok(as_integer) = parse::<u32>(td) {
         print_bits(td, as_integer);
         decoded_one = true;
      }
   }
   if !decoded_one {
      println!("Sorry, that wasn't a number");
      // TODO print this
      // app.print_long_help();
   }
}

fn print_bits(raw_string: &str, number: u32) {
   println!("\n\"{}\" -> {} {:#X} {:#b}", raw_string, number, number, number);

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

