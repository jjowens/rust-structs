mod structs;

struct Employee {
    first_name: String,
    last_name: String,
    role: String,
    department: String,
}

fn main() {
    let _alpha = Employee {
        first_name: "Peter".to_string(),
        last_name: "Smith".to_string(),
        role: "Teacher".to_string(),
        department: "English".to_string(),
    };

    print_employee(_alpha);

    print_employee(create_employee("John".to_string(), "Smith".to_string(), "Chemistry".to_string(), "Science".to_string()));
    print_employee(create_employee("Bob".to_string(), "The Builder".to_string(), "Handy Man".to_string(), "Construction".to_string()));
}

fn create_employee(first_name: String, last_name: String, role: String, department: String) -> Employee {
    Employee {
        first_name,
        last_name,
        role,
        department
    }
}

fn print_employee(employee: Employee) {
    println!(
        "{} {}. Role: {}. Department: {}",
        employee.first_name, employee.last_name, employee.role, employee.department
    );
}
