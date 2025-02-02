fn main(){
   let mut num:i32 = 15;
   earth(&mut num);
   println!("The value of num is:{}",num); 
}
fn earth(number:&mut i32){
   *number = 10 / 2;
   let num = 20 / 2;
   println!("The value of number is:{}",number); 
}
