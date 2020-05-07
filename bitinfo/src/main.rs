use clap::{Arg, App, AppSettings};

fn main() {
   let options = App::new("Trailing args example")
      .setting(AppSettings::TrailingVarArg)
      // TODO more args
      .arg(Arg::with_name("inputs")
           .multiple(true)
          )
      .get_matches();

   // FIXME rm
   println!("Hello, world!");

   println!("{:?}", options);
   let to_decode: Vec<&str> = options.values_of("inputs").unwrap().collect();
   println!("{:?}", to_decode);

   for td in to_decode {
      if let Ok(as_integer) = td.parse::<u32>() {
         println!(" {:?} -> {}", td, as_integer);
      }
   }
}
