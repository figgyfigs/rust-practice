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
    println!("Welcome, loading employee system...");
    println!();

    //Create vector and Hashmap
    //Hashmap Key will be a String and Value will contain a Vector of employees names
    let mut departments: HashMap<String, Vec<&str>> = HashMap::new();

    //let new_dept = String::from("Engineering");
    //let mut my_vec = vec!["Alan", "Jen", "Satoshi"];


    //departments.insert("Engineering".to_string(), my_vec);
    //departments.insert("Engineering".to_string(), vec!["Alan".to_string(), "John".to_string()]);


    for (key, value) in &departments {
        println!("{}: {:?}", key, value);
    }

    loop {
        
        display_ops();
        let user_input = get_opcode();

        //Matches desired command user inputted. This match will call functions
        //TODO: Implement functions for each valid command
        match user_input {
            Some(OpCode::ListDepartments) => list_departments(),
            Some(OpCode::AddEmployee) => add_employee(&mut departments),
            Some(OpCode::ListDepartmentEmployees) => println!("List employees in x department"),
            Some(OpCode::AllEmployees) => println!("List all employees in the company"),
            Some(OpCode::Exit) => println!("Exit"),
            Some(OpCode::InvalidOp) => println!("Invalid OpCode"),
            None => println!("Invalid"),
        }
    }
}

fn display_ops() {
    println!("-------------------------------------------------");
    println!("Welcome! What would you like to do?");
    println!("Press 1: To see a list of departments");
    println!("Press 2: To add an employee to a department");
    println!("Press 3: To list employees in a department");
    println!("Press 4: To list all employees in the company");
    println!("Press 0: Exit");
}

fn get_opcode() -> Option<OpCode> {
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

fn add_employee(departments: &mut HashMap<String, Vec<String>>) {
    println!("Follow the template to add an employee to a department: ");
    println!("Add {{Name}} to {{Company Department}}");
    println!("Example: Add John to Sales");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read the line.");
    
    //Indexes that we need are v[1] and v[3] these will be signed to name and department variables
    let v: Vec<_> = user_input.split_whitespace().collect();
    let employee_name = v[1];
    let department_name = v[3];

    match departments.is_empty() {
        true => println!("Hashmap is empty"),
        false => println!("HashMap is not empty"),
    };

    departments.insert(department_name.to_owned(), vec![employee_name]);

    println!("Adding {} to {}", employee_name, department_name);

}







use std::collections::HashMap;
use std::io;

fn main() {

  let mut departments: HashMap<String, Vec<String>> = HashMap::new();
  // let mut employees = vec![String::from("Alan"), String::from("John"), String::from("Eric")];
  //let mut employees: Vec<String> = Vec::new();

  //departments.insert(String::from("Sales"), employees);
  // departments.insert(String::from("Sales"), vec![String::from("Alan"), String::from("John")]);
  // departments.insert(String::from("Sales"), vec!["Alan".to_string(), "John".to_string()]);

  departments = add_employee(departments);

  for (key, value) in &departments {
    println!("{}: {:?}", key, value);
  }
}

fn add_employee(mut hashmap: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
  let mut vec_to_add: Vec<String> = Vec::new();

  println!("Follow the template to add an employee to a department: ");
  println!("Add {{Name}} to {{Company Department}}");
  println!("Example: Add John to Sales");

  let mut user_input = String::new();
  io::stdin().read_line(&mut user_input).expect("Failed to read the line.");

  //Indexes that we need are v[1] and v[3] these will be signed to name and department variables
  let v: Vec<_> = user_input.split_whitespace().collect();
  let employee_name = v[1];
  let department_name = v[3];

  vec_to_add.push(employee_name.to_string());

  hashmap.insert(department_name.to_string(), vec_to_add);

  hashmap
}