/* Using a hash map and vectors, create a text interface to allow a user to add 
employee names to a department in a company. For example, “Add Sally to Engineering”
 or “Add Amir to Sales.” Then let the user retrieve a list of all people 
in a department or all people in the company by department, sorted alphabetically.
*/

use std::io;
use std::collections::HashMap;

enum OpCode {
    Exit,
    ListDepartments,
    AddEmployee,
    ListDepartmentEmployees,
    AllEmployees,
    InvalidOp,
}

fn main() {
    println!("Welcome... loading employee system");

    //Create vector and Hashmap
    //Hashmap Key will be a String and Value will contain a Vector of employees names
    let mut my_vec = vec!["Alan", "Jen", "Satoshi"];
    let mut departments = HashMap::new();
    let new_dept = String::from("Engineering");

    departments.insert(&new_dept, my_vec);

    for (key, value) in &departments {
        println!("{}: {:?}", key, value);
    }

    loop {
        
        display_ops();
        let user_input = get_input();

        //Matches desired command user inputted. This match will call functions
        //TODO: Implement functions for each valid command
        match user_input {
            Some(OpCode::ListDepartments) => list_departments(),
            Some(OpCode::AddEmployee) => println!("Add a employee"),
            Some(OpCode::ListDepartmentEmployees) => println!("List employees in x department"),
            Some(OpCode::AllEmployees) => println!("List all employees in the company"),
            Some(OpCode::Exit) => println!("Exit"),
            Some(OpCode::InvalidOp) => println!("Invalid OpCode"),
            None => println!("Invalid"),
        }
    }
}

    //departments.insert("Sales", my_vec);
    //departments.insert("Sales", my_vec);

    //println!("{:?}", departments);


fn display_ops() {
    println!("Welcome! What would you like to do?");
    println!("Press 1: To see a list of departments");
    println!("Press 2: To add an employee to a department");
    println!("Press 3: To list employees in a department");
    println!("Press 4: To list all employees in the company");
    println!("Press 0: Exit");
}

fn get_input() -> Option<OpCode> {
    let mut op_code = String::new();

    io::stdin().read_line(&mut op_code).expect("Failed to read the line.");

    let op_code: OpCode = match op_code.trim().parse() {
        Ok(1) => OpCode::ListDepartments,
        Ok(2) => OpCode::AddEmployee,
        Ok(3) => OpCode::ListDepartmentEmployees,
        Ok(4) => OpCode::AllEmployees,
        Ok(0) => OpCode::Exit,
        Ok(_) => OpCode::InvalidOp,
        Err(_) => OpCode::InvalidOp,
    };
    
    Some(op_code)
}

fn new_department() {
    
}

fn list_departments() {
    println!("listing departments...");
}

fn add_employee() {
    
}