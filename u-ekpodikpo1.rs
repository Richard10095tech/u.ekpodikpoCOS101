// a program of the student council for only 50 eligible candidates
// checking requirement for who one who is qualified to vote

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();

    println!{"Enter your name: "};
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter your email: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");

    println!("Enter your department: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");

    println!{"Enter your state of origin: "};
    io::stdin().read_line(&mut input4).expect("Not a valid string");

    println!("Enter your level: ");
    io::stdin().read_line(&mut input5).expect("not a valid string");

    println!("Enter your CGPA: ");
    io::stdin().read_line(&mut input6).expect("not a valid string");

    println!("Enter your class position: ");
    io::stdin().read_line(&mut input7).expect("not a valid string");

    if{ not in 100 level and has a CGPA > 4.0 and meets all the requirements{
    println!("You can vote {}!", input1, input2, input3, input4, input5, input6, input7) 
    }
    } else {
       println!("Sorry, you can not vote {}", input1, input2, input3, input4, input5, input6, input7)
    }
}