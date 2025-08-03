mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.iter())
        .max_by_key(|(_, store)| store.square_meters)
        .map(|(name, store)| (name.clone(), store.clone()))
        .unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, Employee)> {
    let mut max_salary = 0.0;
    let mut top_earners = Vec::new();

    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            for (employee_name, employee) in &store.employees {
                if employee.salary > max_salary {
                    max_salary = employee.salary;
                    top_earners.clear();
                    top_earners.push((employee_name.as_str(), *employee));
                } else if employee.salary == max_salary {
                    top_earners.push((employee_name.as_str(), *employee));
                }
            }
        }
    }

    top_earners
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut employee_count = mall.guards.len();

    for (_, floor) in &mall.floors {
        for (_, store) in &floor.stores {
            employee_count += store.employees.len();
        }
    }

    employee_count
}

pub fn check_for_securities(mall: &mut Mall, available_guards: HashMap<String, Guard>) {
    let mut total_floor_area = 0;

    for (_, floor) in &mall.floors {
        total_floor_area += floor.size_limit;
    }

    let required_guards = total_floor_area / 200;
    let current_guards = mall.guards.len() as u64;

    if required_guards <= current_guards {
        return;
    }

    let mut guards_needed = required_guards - current_guards;

    for (guard_name, guard) in available_guards {
        if guards_needed == 0 {
            break;
        }

        mall.hire_guard(&guard_name, guard);
        guards_needed -= 1;
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let hours = employee.working_hours.1 - employee.working_hours.0;
                if hours >= 10 {
                    employee.raise(employee.salary * 0.10);
                } else {
                    employee.cut(employee.salary * 0.10);
                }
            }
        }
    }
}
