/* Using a hash map and vectors, create a text interface to allow a user to add 
employee names to a department in a company. For example, “Add Sally to Engineering”
 or “Add Amir to Sales.” Then let the user retrieve a list of all people 
in a department or all people in the company by department, sorted alphabetically.
*/

use std::io;
use std::collections::HashMap;
use std::process;

enum OpCode {
    Exit,
    AddEmployee,
    ListDepartmentEmployees,
    ShowAll,
    InvalidOp,
}

fn main() {
    println!("Welcome, loading employee system...");
    println!();

    //Create vector and Hashmap
    //Hashmap Key will be a String and Value will contain a Vector of employees names
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();

    departments.insert("Sales".to_string(), vec!["John".to_string(), "Joe".to_string(), "Ana".to_string(), "Elon".to_string(), "Henry".to_string()]);
    departments.insert("Engineering".to_string(), vec!["Luis".to_string(), "Mart".to_string(), "Will".to_string(), "Noe".to_string(), "Hert".to_string()]);
    departments.insert("Human Resources".to_string(), vec!["Jake".to_string(), "Kirin".to_string(), "Bob".to_string(), "Noely".to_string(), "Hutson".to_string()]);

    loop {
        
        display_ops();
        let user_input = get_opcode();

        //Matches desired command user inputted. This match will call functions
        //TODO: Implement functions for each valid command
        match user_input {
            Some(OpCode::AddEmployee) => {
                departments = add_employee(departments);

                for (key, value) in &departments {
                    println!("{}: {:?}", key, value);
                }
            },
            Some(OpCode::ListDepartmentEmployees) => display_department(&departments),
            Some(OpCode::ShowAll) => println!("List all employees in the company"),
            Some(OpCode::Exit) => {
                println!("Exiting employee system... goodbye.");
                process::exit(1);
            },

            Some(OpCode::InvalidOp) => println!("Invalid OpCode"),

            None => println!("Please enter a vaild code."),
        }
    }
}

fn display_ops() {
    println!("-------------------------------------------------");
    println!("Welcome! What would you like to do?");
    println!("Enter 1: To add an employee to a department");
    println!("Enter 2: To list employees in a department");
    println!("Enter 3: To list all employees in the company");
    println!("Enter 0: To exit");
}

fn new_department() {
    
}


//Show everything in the company directory. Name and department (Alphabetical order by name)
fn show_all() {

}

/* 
Shows the list of departments available.
Promt user to choose a department they would like to show 
Accept user input and display all employees that are assigned to that department (alphabetical order)
*/
fn display_department(map: &HashMap<String, Vec<String>>) {
    println!("Loading departments...");

    let department_keys = map.keys().cloned()
        .collect::<Vec<String>>().sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    // let new_vec = department_keys.sort();
    // println!("Modified vec is {:?}", new_vec);

    if department_keys.is_empty() {
        println!("No departments in the company directory.");
    } else {
        for(i, name) in department_keys.iter().enumerate() {
            println!("Enter {} to show the {} Department", i, name);
        }
    }

    println!(); 
    println!("What department would you like to show?");
    
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read the line.");
    let index: usize = user_input.trim().parse().expect("Input is not an integer");

    let selected_dept = department_keys[index].to_string();

    match map.get(&selected_dept) {
        Some(name) => println!("{}: {:?}", selected_dept, name),
        None => println!("Department is empty")
    }

    //println!("{:?}", map.get_key_value(&selected_dept));
    //println!("{:?}", map.get(&selected_dept));

}







fn add_employee(mut hashmap: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut employee_vec: Vec<String> = Vec::new();

    println!("Follow the template to add an employee to a department: ");
    println!("Add {{Name}} to {{Company Department}}");
    println!("Example: Add John to Sales");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read the line.");
    
    //Indexes that we need are v[1] and v[3] these will be signed to name and department variables
    let v: Vec<_> = user_input.split_whitespace().collect();
    let employee_name = v[1];
    let department_name = v[3];

    match hashmap.is_empty() {
        true => println!("Hashmap is empty"),
        false => println!("HashMap is not empty"),
    };

    employee_vec.push(employee_name.to_string());

    hashmap.insert(department_name.to_string(), employee_vec);

    println!("Adding {} to {}", employee_name, department_name);

    hashmap

}

fn get_opcode() -> Option<OpCode> {
    let mut op_code = String::new();

    io::stdin().read_line(&mut op_code).expect("Failed to read the line.");

    let op_code: OpCode = match op_code.trim().parse() {
        Ok(1) => OpCode::AddEmployee,
        Ok(2) => OpCode::ListDepartmentEmployees,
        Ok(3) => OpCode::ShowAll,
        Ok(0) => OpCode::Exit,
        Ok(_) => OpCode::InvalidOp,
        Err(_) => OpCode::InvalidOp,
    };
    
    Some(op_code)
}
