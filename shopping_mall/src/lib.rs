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
    let mut max_salary = 0.0;
    let mut result = Vec::new();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (emp_name, emp) in &store.employees {
                if emp.salary > max_salary {
                    max_salary = emp.salary;
                    result.clear();
                    result.push((emp_name.clone(), *emp));
                } else if (emp.salary - max_salary).abs() < f64::EPSILON {
                    result.push((emp_name.clone(), *emp));
                }
            }
        }
    }

    result
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let employee_count: usize = mall
        .floors
        .values()
        .map(|floor| {
            floor
                .stores
                .values()
                .map(|store| store.employees.len())
                .sum::<usize>()
        })
        .sum();

    employee_count + mall.guards.len()
}

pub fn check_for_securities(mall: &mut Mall, mut available_guards: HashMap<String, Guard>) {
    let total_area: u64 = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.square_meters)
        .sum();

    let required_guards = (total_area as f64 / 200.0).ceil() as usize;
    let current_guards = mall.guards.len();

    let to_add = required_guards.saturating_sub(current_guards);

    for (name, guard) in available_guards.drain().take(to_add) {
        mall.hire_guard(name, guard);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for emp in store.employees.values_mut() {
                let hours = emp.working_hours.1 - emp.working_hours.0;
                if hours >= 10 {
                    emp.raise(emp.salary * 0.10);
                } else {
                    emp.cut(emp.salary * 0.10);
                }
            }
        }
    }
}
