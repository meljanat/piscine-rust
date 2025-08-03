pub mod mall;
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

pub fn highest_paid_employee(mall: &Mall) -> Vec<(String, Employee)> {
    let mut result = vec![];
    let mut max_salary = 0.0;

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if employee.salary > max_salary {
                    max_salary = employee.salary;
                    result = vec![(name.clone(), *employee)];
                } else if (employee.salary - max_salary).abs() < f64::EPSILON {
                    result.push((name.clone(), *employee));
                }
            }
        }
    }

    result
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.employees.len())
        .sum()
}

pub fn check_for_securities(mall: &mut Mall, mut guard_pool: HashMap<String, Guard>) {
    let total_area: u64 = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.square_meters)
        .sum();

    let required_guards = ((total_area + 199) / 200) as usize;

    for (name, guard) in guard_pool.drain() {
        if mall.guards.len() >= required_guards {
            break;
        }
        mall.hire_guard(name, guard);
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
