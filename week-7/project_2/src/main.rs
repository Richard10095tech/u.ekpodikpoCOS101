// Rust program to checker for the highest years of experience of programers

use std::io;

fn checker(){
    let mut input = String::new();
    println!("Enter your years of experience");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _years:i32 = input.trim().parse().expect("Invalid input");

    if _years > 10 // you have more than 10 years experience
    {
        println!("You can be selected as the programer with the highest years of experience");
    }
    else {
        println!("You can never be selected as the programer with the highest years of experience");
    }

}

fn main() {
   // calling function
   println!("Welcome! this program checks for the programer with the highest years of experience");
   checker()
}
