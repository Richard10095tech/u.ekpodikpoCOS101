
struct Laptops {
    brand: String,
    price: u32,
    quantity: u32
}
 fn main() {
    let laptop1 = Laptops{
        brand: "HP".to_string(),
        price: 650_000, // per HP laptop 
        quantity: 3,
    };

    let mut total_cost1 = 0;
    
        total_cost1 += laptop1.price * laptop1.quantity;
    
    println!("Total cost for the customer is {}",total_cost1);

     let laptop2 = Laptops{
        brand: "IBM".to_string(),
        price: 755_000, // per IBM laptop
        quantity: 3, 
    };

    let mut total_cost2 = 0;
    
        total_cost2 += laptop2.price * laptop2.quantity;
    
    println!("Total cost for the customer is {}",total_cost2);

    let laptop3 = Laptops{
        brand: "TOSHIBA".to_string(),
        price: 550_000, // per TOSHIBA laptop
        quantity: 3,
    };

    let mut total_cost3 = 0;
    
        total_cost3 += laptop3.price * laptop3.quantity;
    
    println!("Total cost for the customer is {}",total_cost3);

     let laptop4 = Laptops{
        brand: "DELL".to_string(),
        price: 850_000, // per DELL laptop
        quantity: 3,
    };

    let mut total_cost4 = 0;
    
        total_cost4 += laptop4.price * laptop4.quantity;
    
    println!("Total cost for the customer is {}",total_cost4);
    
    let mut _total_cost_of_all_3_brands_together = 0;
         _total_cost_of_all_3_brands_together = total_cost1 + total_cost2 + total_cost3 + total_cost4;
    println!("The compiled total cost for the 12 laptops of the customer is {}",_total_cost_of_all_3_brands_together);      

    display(laptop1);
    display(laptop2);
    display(laptop3);
    display(laptop4);
 }
 fn display( laptop:Laptops){
    println!("brand is {} price is {} quantity is {}",laptop.brand,laptop.price,laptop.quantity);
 }