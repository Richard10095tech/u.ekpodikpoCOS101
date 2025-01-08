fn main() {
    println!("Welcome! \nThis program is a Public Service APS level checker for FGN. \nKindly input your position and years of experience.");
      
    // To check the APS level of someone in the field of office administration

    // Create an empty vector "Work Level"
    let work_level : Vec<String> = Vec::new();
    // Print Work level vector
    println!("THe Work Level vector has elements {}",work_level.len());
    // push new element into
    let mut input1 = String::new();
    println!("What is your work level");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let _work_level:String = input1.trim().parse().expect("Invalid input");


    let mut input2 = String::new();
    println!("Enter work level (Intern, Admimistrator, Senior Admimistrator, Office Manager, Director, CEO");
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let _work_level:String = input2.trim().parse().expect("Invalid input");


    println!("Your APS level is:\n SES");
    let work_level: Vec<String> = Vec::new();
    // loop to iterate elements in vector
    for r in &work_level.clone()
    {
       //iterating through r on the vector
       println!("{:?} {:?}", work_level.clone(), r);
       let _ = work_level.clone();
    }
    
    // To check the APS level of someone in the field of academics.

    let work_level : Vec<String> = Vec::new();
    println!("THe Work Level vector has elements {}",work_level.len());

    let mut input1 = String::new();
    println!("What is your work level");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let _work_level:String = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter work level (Research Assistant, PhD Candidate, Post-Doc Reseaercher, Senior Lecturer, Dean)");
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let _work_level:String = input2.trim().parse().expect("Invalid input");
    
    println!("Your APS level is:\n SES");
    let work_level: Vec<String> = Vec::new();
    // loop to iterate elements in vector
    for r in &work_level.clone()
    {
       //iterating through r on the vector
       println!("{:?} {:?}", work_level.clone(), r);
       let _ = work_level.clone();
    }

    // To check the APS level of someone in the field of defending people in court(lawyer)

    let work_level : Vec<String> = Vec::new();
    println!("THe Work Level vector has elements {}",work_level.len());

    let mut input1 = String::new();
    println!("What is your work level");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let _work_level:String = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter work level (Paralegal, Junior Associate, Associate, Senior Associate 1 - 2, Senior Associate 3 - 4, Partner)");
    std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let _work_level:String = input2.trim().parse().expect("Invalid input");
    
    println!("Your APS level is:\n SES");
    let work_level: Vec<String> = Vec::new();
    // loop to iterate elements in vector
    for r in &work_level.clone()
    {
       //iterating through r on the vector
       println!("{:?} {:?}", work_level.clone(), r);
       let _ = work_level.clone();
    }

    // To check for the APS level for someone that is in the field of Teaching

    let work_level : Vec<String> = Vec::new();
    println!("THe Work Level vector has elements {}",work_level.len());

    let mut input1 = String::new();
    println!("What is your work level");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let _work_level:String = input1.trim().parse().expect("Invalid input");

    let mut input2 = String::new();
    println!("Enter work level (Placement, Classroom Teacher, Senior Teacher, Leading Teacher, Deputy Principal, Principal)");
     std::io::stdin().read_line(&mut input2).expect("Failed to read input");
    let _work_level:String = input2.trim().parse().expect("Invalid input");
    
    println!("Your APS level is:\n SES");
    let work_level: Vec<String> = Vec::new();
    // loop to iterate elements in vector
    for r in &work_level.clone()
    {
       //iterating through r on the vector
       println!("{:?} {:?}", work_level.clone(), r);
       let _ = work_level.clone();
    }
}   
 
