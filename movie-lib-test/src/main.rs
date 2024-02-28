extern crate movies_lib;
use movies_lib::movies::play;
use movies_lib::movies::typename;
fn main() {
   println!("inside main of test ");
   play("Tutorialspoint".to_string());

   typename();
   
}