pub mod helper {
    use crate::Employee;

    pub fn create_employee(first_name: String, last_name: String, role: String, department: String) -> Employee {
        Employee {
            first_name,
            last_name,
            role,
            department
        }
    }

    pub fn print_employee(employee: Employee) {
        println!(
            "{} {}. Role: {}. Department: {}",
            employee.first_name, employee.last_name, employee.role, employee.department
        );
    }

}