use std::io;
use std::collections::HashMap;

/* Using a hash map and vectors, create a text interface to allow a user to add employee names
to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.”
Then, let the user retrieve a list of all people in a department or all people in the company
by department, sorted alphabetically. */

fn interface() {

    let mut company: HashMap<String, Vec<String>> = HashMap::new();

    println!("Welcome to your company employee system. Type 'help' to view commands.\n");

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        let command_vec: Vec<&str> = command.trim().split_whitespace().collect();

        match command_vec.as_slice() {

            ["help"] => {
                println!(
                    "======================= Page 1 of 2 =======================\n\
                    create [department]:        Creates department with specified name.\n\
                    add [name] to [department]: Adds employee with specified name to specified department.\n\
                    list all:                   Lists all employees alphabetically by department.\n\
                    list [department]:          Lists all employees in the specified department alphabetically.\n\
                    exit:                       Exits the program. Remember to save your changes!"
                );
            },

            ["create", department] => {
                company.entry(String::from(*department)).or_insert_with(Vec::new);
                println!("Created {} department.\n", department);
            },

            ["add", name, "to", department] => {
                match company.get_mut(*department) {
                    Some(people) => {
                        people.push(name.to_string());
                        println!("{} was added to {}.\n", name, department);
                    },

                    None => {println!("That department does not exist.\n")},
                }
            },

            ["list", "all"] => {
                let mut departments: Vec<String> = company.keys().cloned().collect();
                departments.sort();

                for department in departments {
                    println!("{}", department);
                    match company.get(&department) {
                        Some(people) => {
                            if people.is_empty() {
                                println!("  EMPTY");
                            } else {
                                let mut employees = people.clone();
                                employees.sort();
                                for name in employees{
                                println!("  {}", name);
                                }
                            }
                        },

                        None => {panic!("Something went wrong.");},
                    }
                }

            },

            ["list", department] => {
                match company.get(*department) {
                    Some(people) => {
                        if people.is_empty() {
                            println!("{} has no employees.\n", department);
                        } else {
                            let mut employees = people.clone();
                            employees.sort();
                            for name in employees{
                            println!("{}", name);
                            }
                        }
                    },

                    None => {println!("The {} department has not been created.\n", department);},
                }
                
            },

            ["exit"] => {
                println!("Oops, you forgot to save. Your changes were lost.\n");
                break;
            },

            _ => {
                println!("Unknown command. Please try again.\n");
            },
        }
    }
    
}

fn main() {
    interface();
}
