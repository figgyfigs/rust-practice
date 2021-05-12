/* Using a hash map and vectors, create a text interface to allow a user to add 
employee names to a department in a company. For example, “Add Sally to Engineering”
 or “Add Amir to Sales.” Then let the user retrieve a list of all people 
in a department or all people in the company by department, sorted alphabetically.
*/

use std::io;

enum OpCode {
    Exit,
    ListDepartments,
    AddEmployee,
    ListDepartmentEmployees,
    AllEmployees,  
}

fn main() {
    display_ops();
    let user_input = get_input();

    match user_input {
        1 => println!("One!!"),
        2 => println!("Two!!"),
        3 => println!("Three!!"),
        4 => println!("Four!!"),
        _ => println!("Invalid"),
    }

    //println!("User typed... {}", user_input);
}

fn display_ops() {
    println!("Welcome! What would you like to do?");
    println!("Press 1: To see a list of departments");
    println!("Press 2: To add an employee to a department");
    println!("Press 3: To list employees in a department");
    println!("Press 4: To list all employees in the company");
    println!("Press 0: Exit");
}

fn get_input() -> i32 {
    let mut op_code = String::new();

    io::stdin().read_line(&mut op_code).expect("Failed to read the line.");

    let op_code: i32 = match op_code.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };
    
    op_code
}