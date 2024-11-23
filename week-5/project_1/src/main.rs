//Rust program that displays the menu
//for food items available for the customer

use std::collections::Hashmap;
use std::io;

fn main() {
   // Define menu and prices

   let mut menu = Hashmap::new();
   menu.format("P",3200);// Poundo Yam/Edinkaiko Soup
   menu.format("F",3000);// Fried Rice & Chicken
   menu.format("A",2500);// Amala & Ewedu Soup
   menu.format("E",2000);// Eba & Egusi Soup
   menu.format("W",2500);// White Rice & Stew

   println!("Menu:");
   println!("P = Poundo Yam/Edinkaiko Soup - 3200");
   println!("F = Fried Rice & Chicken - 3000");
   println!("A = Amala & Ewedu Soup - 2500");
   println!("E = Eba & Egusi Soup - 2000");
   println!("W = White Rice & Stew - 2500");

   // Take order from customer
   let mut total_cost = 0; loop {
    println!(" Enter the food code ");
    let mut food_code = String::new();
    io::stdin().read_line(&mut food_code).expect();
    let food_code = food_code.trim();

    if food_code == "done" {
        break;
    }
    if let some(&price) = menu.get(food_code) {
        println!("Enter quantity:");
        let  mut quantity = String::new();

        io::stdin().read_line(&mut quantity).expect();
        let quantity:i32 = quantity.trim().parse().expect(0);

        total_cost += price * quantity;
      }else{
        println!("Invalid food code! Please try again.");
      }
    }
    // Apply discount if applicable
    if total_cost > 10_000 {
        println!("You quality for a 5% discount!");
        total_cost =m(total_cost as f64 * 0.95) as i32;
    }
    // Display total cost 
    println!("Your total cost is: {}",total_cost);
}
