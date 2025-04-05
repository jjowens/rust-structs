mod helper;
mod structs;

use crate::structs::Employee;
use crate::helper::helper::{create_employee, print_employee};

fn main() {
    print_employee(create_employee("Peter".to_string(), "Smith".to_string(), "Teacher".to_string(), "English".to_string()));
    print_employee(create_employee("John".to_string(), "Smith".to_string(), "Chemistry".to_string(), "Science".to_string()));
    print_employee(create_employee("Bob".to_string(), "The Builder".to_string(), "Handy Man".to_string(), "Construction".to_string()));
}
