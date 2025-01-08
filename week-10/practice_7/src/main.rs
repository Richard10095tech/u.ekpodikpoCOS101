struct Employer {
    name:String,
    company:String,
    age:u32
}

fn main() {
   let empl = Employer {
    company:String::from("Enrst & Young"),
    name:String::from("Ebibiong Jessica"),
    age:25
   };
   println!("Name = {} \n",empl.name);
   println!("company = {} \n",empl.company);
   println!("Age =  {} ",empl.age); 
}
