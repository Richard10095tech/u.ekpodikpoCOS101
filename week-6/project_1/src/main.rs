// Rust program to perform calculations

use std::io;

// Choose a formula that you want to use.

fn checker(){
    let mut input = String::new();
    println!("Enter a Formula");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _area:String = input.trim().parse().expect("Invalid input");

    if _area == "T" // Area of a trapezium.
    {
    println!("Enter height");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _area:String = input.trim().parse().expect("Invalid input");

    println!("Enter base1");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _area:String = input.trim().parse().expect("Invalid input");

    println!("Enter base2");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _area:String = input.trim().parse().expect("Invalid input");

    println!("Result = height/2 * base1 + base2");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let _area:String = input.trim().parse().expect("Invalid input");

   }  
    if _area == "R"  // Area of a rhombus.
    {
        println!("Enter diagonal1");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _area:String = input.trim().parse().expect("Invalid input");

        println!("Enter diagonal2");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _area:String = input.trim().parse().expect("Invalid input");

        println!("Reasult = 1/2 * diagonal1 * diagonal2");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _area:String = input.trim().parse().expect("Invalid input");

    }
    if _area == "P"  // Area of a parallelogram.
    {
        println!("Enter base");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _area:String = input.trim().parse().expect("Invalid input");

        println!("Enter altitude");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _area:String = input.trim().parse().expect("Invalid input"); 

        println!("Result = base * altitude");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _area:String = input.trim().parse().expect("Invalid input"); 
 
    }
    if area == "C"  // Area of a cube.
    {
        println!("Enter lenght of the side");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _area:String = input.trim().parse().expect("Invalid input"); 

        println!("Result = 6 * (lenght of the side)");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _area:String = input.trim().parse().expect("Invalid input"); 


    }
    if area == "V"  // Volume of a cylinder.
    {
        println!("Enter radius");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _area:String = input.trim().parse().expect("Invalid input"); 

        println!("Enter height");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _area:String = input.trim().parse().expect("Invalid input"); 

        println!("Result = π * radius^2 * height");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let _area:String = input.trim().parse().expect("Invalid input"); 

    }
    else{
       println!("You can check any formula that you will like to use"); 
    }
}

fn main() {
    // calling function
    println!("Welcome! this program checks for any formular that you want");
    checker()
} 
