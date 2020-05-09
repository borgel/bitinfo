use clap::{Arg, App, AppSettings};
use parse_int::parse;

fn main() {
   let options = App::new("Trailing args example")
      .setting(AppSettings::TrailingVarArg)
      // TODO more args
      .arg(Arg::with_name("inputs")
           .multiple(true)
          )
      .get_matches();

   println!("{:?}", options);
   let to_decode: Vec<&str> = options.values_of("inputs").unwrap().collect();
   println!("{:?}", to_decode);

   for td in to_decode {
      if let Ok(as_integer) = parse::<u32>(td) {
         print_bits(td, as_integer)
      }
   }
}

fn print_bits(raw_string: &str, number: u32) {
   println!("{} -> {}", raw_string, number);
}

