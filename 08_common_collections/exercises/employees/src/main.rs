use std::collections::HashMap;
use std::io;

fn main() {
    let mut db: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("[1] Add employee to department");
        println!("[2] List employees in department");
        println!("[3] List all employees");
        println!("[anything else] Quit");

        let mut cmd = String::new();

        io::stdin()
            .read_line(&mut cmd)
            .expect("Failed to read input");

        cmd = match cmd.trim().parse() {
            Ok(input_cmd) => input_cmd,
            Err(_) => continue,
        };

        match cmd.as_ref() {
            "1" => add_employee(&mut db),
            "2" => list_employees_in_department(&db),
            "3" => list_all_employees(&db),
            _ => {
                println!("See Ya!");
                break;
            }
        }
        println!();
    }
}

fn add_employee(db: &mut HashMap<String, Vec<String>>) {
    println!("Please input the name of the employee.");
    let mut employee = String::new();

    io::stdin()
        .read_line(&mut employee)
        .expect("Failed to read lines");

    println!("Please input the department.");
    let mut department = String::new();

    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read lines");

    employee = String::from(employee.trim());
    department = String::from(department.trim());
    db.entry(department).or_insert(Vec::new()).push(employee);

    println!("Employee was added!");
    println!();
}

fn list_employees_in_department(db: &HashMap<String, Vec<String>>) {
    println!("Please input the department.");
    let mut department = String::new();

    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read lines");

    let department = department.trim();
    match db.get(department) {
        Some(employees) => {
            let mut employees = employees.clone();
            employees.sort();

            println!();
            println!("All employees in {department}:");

            for employee in employees {
                println!("* {employee}");
            }
        }
        None => println!("Department {department} not found"),
    }
}

fn list_all_employees(db: &HashMap<String, Vec<String>>) {
    for (department, employees) in db {
        let mut employees = employees.clone();

        println!("{department}");

        employees.sort();
        for employee in employees {
            println!("* {employee}");
        }
    }
}
