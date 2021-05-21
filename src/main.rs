use bitcask::*;

use std::string::String;
use bitcask::error::Error;

fn main() {
   println!("init project");

   let c = Error::ParseError{
      err: "errr".into(),
      code: 16,
   };

   std::io::Error;

   println!("rc: {:?}", c);
}


