use std::fs;

fn main() {
   fs::remove_file("data.txt").expect("could not renove file");
   println!("file is removed");
}
