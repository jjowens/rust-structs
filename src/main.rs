mod helper;
mod structs;

use crate::structs::structs::Employee;
use crate::helper::helper::{create_employee, print_employee};

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
