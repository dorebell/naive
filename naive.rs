use std::env;
use std::process::exit;

const CORRECT_PASSWORD: &'static [u8] = "m".as_bytes();

fn main() {
    let arg = env::args().nth(1).expect("argument is provided");
    let user_password: &[u8] = arg.as_bytes();
    if user_password == CORRECT_PASSWORD {
         //println!("{}", 0);
         exit(0);
    } else {
         //println!("{}", 1);
         exit(1);
    }
}