fn main(){
   let index:i32 = 5;
   mutate_no_to_zero(index);
   println!("The value of index is:{}",index);
}

fn mutate_no_to_zero(mut param_index: i32) {
   param_index = param_index*5;
   println!("param_index value is :{}",param_index); 
}