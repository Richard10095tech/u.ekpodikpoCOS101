use std::fs::File;
use std::io::Write;

fn main(){
    // Define a struct to hold student details
    struct Student {
        name: String,
        matric_number: String,
        department: String,
        level: u32,

    }

    // Create a list of students
    let students = vec![
        Student {
            name: String::from("Oluchi Mordi"),
            matric_number: String::from("ACC10211111"),
            department: String::from("Accounting"),
            level: 300,

        },
        Student {
            name: String::from("Adams Aliyu"),
            matric_number: String::from("ECO10110101"),
            department: String::from("Economics"),
            level: 100,
        },
        Student {
            name: String::from("Shania Bolade"),
            matric_number: String::from("CSC10328828"),
            department: String::from("Computer"),
            level: 200,
        }, 
        Student {
            name: String::from("Blanca Edewoh"),
            matric_number: String::from("HEE10202001"),
            department: String::from("Mechanical"),
            level: 100,
        },
        
    ];

    // Display and save the details
    let mut file = File::create("Student_details.txt").expect("Unable to create file");

    let header = "============================\n PAU STUDENT INFORMATION    \n============================\n";
    println!("{}", header);
    file.write_all(header.as_bytes()).expect("Unable to write to file");

    for (index, student) in students.iter().enumerate() 
    {
           ( index + 1,
             student.name.clone(),
             student.matric_number.clone(),
             student.department.clone(),
             student.level, );
        
        file.write_all( student.department.as_bytes()).expect("Unable to write to file");
        file.write_all( student.matric_number.as_bytes()).expect("Unable to write to file");
        file.write_all( student.name.as_bytes()).expect("Unable to write to file");

    }

    let footer = "End of student information.\n";
    println!("{}", footer);
    file.write_all(footer.as_bytes()).expect("Unable to write to fise")
}