//String_input
use std::io;

fn main(){
  println!("Enter something");
  let mut input_string = String::new();
  io::stdin().read_line(&mut input_string).expect("Failed to read input");
  println!("You entered {}",input_string);
}
