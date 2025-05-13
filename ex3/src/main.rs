use std::io::{self, Write};
use std::collections::HashMap;
fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        print!("Command (Add <employee> to <department>, Remove <employee> from <department>, Show <department>, ShowAll, Quit):\n");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim().to_lowercase();

        if command == "quit" {
            println!("Goodbye!");
            break;
        }
        
        let mut input = command.trim().split_whitespace();

        let Some(action) = input.next() else {
            println!("Invalid command");
            continue;
        };

        match action.to_lowercase().as_str() {
            "add" => {
                let Some(employee) = input.next() else {
                    println!("Invalid command");
                    continue;
                };

                let Some(department) = input.nth(1) else {
                    println!("Invalid command");
                    continue;
                };

                employees.entry(department.to_string())
                    .or_insert(Vec::new())
                    .push(employee.to_string());

                println!("{:?}", employees);
            }
            "remove" => {
                let Some(employee) = input.next() else {
                    println!("Invalid command");
                    continue;
                };

                let Some(department) = input.nth(1) else {
                    println!("Invalid command");
                    continue;
                };
                if let Some(emp_list) = employees.get_mut(department) {
                    emp_list.retain(|e| e != employee);
                }
               
                println!("{:?}", employees);
            }
            "show" => {
                let Some(department) = input.next() else {
                    println!("Invalid command");
                    continue;
                };

                match employees.get(department) {
                    Some(employees) => {
                        let mut sorted_employees = employees.clone();
                        sorted_employees.sort();
                        println!("Employees in {} {:?}", department, sorted_employees);
                    }
                    None => {
                        println!("No employees found in {}", department);
                    }
                }

            }

            "showall" => {
                for (department, employees) in &employees {
                    let mut sorted_employees = employees.clone();
                    sorted_employees.sort();
                    println!("Employees in {} {:?}", department, sorted_employees);
                }
            }

            _ => {
                println!("Invalid command");
            }
        }

    }


}


