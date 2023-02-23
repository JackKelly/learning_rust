use std::io;
use std::collections::HashMap;

fn main() {
    let mut employees = HashMap::new();
    loop {
        println!("Enter one of these two commands:");
        println!("Add <name> to <department>");
        println!("Show");

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line!");

        let words = Vec::from_iter(command.split_whitespace());
        if (words.len() == 1) & (words[0] == "Show") {
            println!("{:?}", employees);
        } else if words[0] == "Add" {
            let employee = words[1].to_string();
            let department = words[3].to_string();
            println!("Adding {} to {} department", employee, department);
            employees.insert(employee, department);
        }
    }
}
