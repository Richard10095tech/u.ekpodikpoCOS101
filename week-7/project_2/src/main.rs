// Rust program to checker for the highest years of experience of programers

use std::io;

fn checker(){
    let mut input = String::new();
    println!("Welcome! how many programers do you want to interview");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _number:i32 = input.trim().parse().expect("Invalid input");

    if _number == 3 // you want to interview the first programers
    {
        println!("Enter the name of first programer");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _number:String= input.trim().parse().expect("Invalid input");

        println!("Enter the amount of years of experience");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _number:i32 = input.trim().parse().expect("Invalid input");

    } 
        if _number == 3 // you to interview the second programer
        {
        println!("Enter the name of the second programer");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _number:String = input.trim().parse().expect("Invalid input");

        println!("Enter the amount of years  of experience");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _number:i32 = input.trim().parse().expect("Invalid input");
        }

         if _number == 3 // you to interview the third programer
        {
        println!("Enter the name of the third programer");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _number:String = input.trim().parse().expect("Invalid input");

        println!("Enter the amount of years  of experience");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _number:i32 = input.trim().parse().expect("Invalid input");
        }

        else {
            println!("If you are selected as the programer with the highest years of experience, you are quqlified for the job");
        }
}

fn main() {
   // calling function
   println!("Welcome! this program checks for the programer with the highest years of experience");
   checker()
}
