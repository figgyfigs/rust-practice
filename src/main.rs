/* Using a hash map and vectors, create a text interface to allow a user to add 
employee names to a department in a company. For example, “Add Sally to Engineering”
 or “Add Amir to Sales.” Then let the user retrieve a list of all people 
in a department or all people in the company by department, sorted alphabetically.
*/

use std::io;

fn main() {
    //println!("Running...");

    let user_input = get_input();

    println!("User typed... {}", user_input);
}

fn get_input() -> String {
    let mut op_code = String::new();
    io::stdin().read_line(&mut op_code)
    .expect("Failed to read the line.");

    return op_code
}