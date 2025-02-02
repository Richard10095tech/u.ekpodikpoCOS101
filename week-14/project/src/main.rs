 use std::io;
 use std::io::Read;

fn show_database_structure(){
      let mut adm = std::fs::File::open("globacom_db.sql").unwrap();
      let mut contents_adm = String::new();
      adm.read_to_string(&mut contents_adm).unwrap();
      println!("{contents_adm}");
}
fn show_project_table_structure(){
      let mut prom = std::fs::File::open("project_tb.sql").unwrap();
      let mut contents_prom = String::new();
      prom.read_to_string(&mut contents_prom).unwrap();
      println!("{contents_prom}");
}
fn show_staff_table_structure(){
      let mut eml = std::fs::File::open("staff_tb.sql").unwrap();
      let mut contents_eml = String::new();
      eml.read_to_string(&mut contents_eml).unwrap();
      println!("{contents_eml}");
}
fn show_customer_table(){
      let mut custo = std::fs::File::open("customer_tb.sql").unwrap();
      let mut contents_custo = String::new();
      custo.read_to_string(&mut contents_custo).unwrap();
      println!("{contents_custo}");
}
fn show_dataplan_table(){
      let mut vend = std::fs::File::open("dataplan_tb.sql").unwrap();
      let mut contents_vend = String::new();
      vend.read_to_string(&mut contents_vend).unwrap();
      println!("{contents_vend}");
}


fn main(){
  println!("choose the role that you want from Administrator, Project_manager, Employee, Customer, Vendor");

    let mut input = String::new();
    println!("User:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let us = input.trim();

    if us == "an_administrator"{
      println!("show_database_structure");
      show_database_structure();
       let mut adm = std::fs::File::open("globacom_db.sql").unwrap();
      let mut contents_adm = String::new();
      adm.read_to_string(&mut contents_adm).unwrap();
      println!("{contents_adm}");

     } else if us == "a_project_manager"{
        println!("show_project_table_structure");
        show_project_table_structure();
        let mut prom = std::fs::File::open("project_tb.sql").unwrap();
        let mut contents_prom = String::new();
        prom.read_to_string(&mut contents_prom).unwrap();
        println!("{contents_prom}");

    } else if us == "an_employee"{
        println!("show_staff_table_structure");
        show_staff_table_structure();
        let mut eml = std::fs::File::open("staff_tb.sql").unwrap();
        let mut contents_eml = String::new();
        eml.read_to_string(&mut contents_eml).unwrap();
        println!("{contents_eml}");

    } else if us == "a_customer"{
        println!("show_customer_table");
        show_customer_table(); 
        let mut custo = std::fs::File::open("customer_tb.sql").unwrap();
        let mut contents_custo = String::new();
        custo.read_to_string(&mut contents_custo).unwrap();
        println!("{contents_custo}");

    } else if us == "a_vendor"{
        println!("show_dataplan_table");
        show_dataplan_table();
        let mut vend = std::fs::File::open("dataplan_tb.sql").unwrap();
        let mut contents_vend = String::new();
        vend.read_to_string(&mut contents_vend).unwrap();
        println!("{contents_vend}");

    } else {
        println!("Therefore, you have inputed a wrong role, Kindly replace it with a correct role by choosing from what is  listed above");
    }
}   

