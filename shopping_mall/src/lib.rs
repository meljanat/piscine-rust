pub mod mall;
pub use mall::*;
use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut biggest: Option<(String, Store)> = None;

    for floor in mall.floors.values() {
        for (store_name, store) in &floor.stores {
            if let Some((_, ref current_biggest)) = biggest {
                if store.square_meters > current_biggest.square_meters {
                    biggest = Some((store_name.clone(), store.clone()));
                }
            } else {
                biggest = Some((store_name.clone(), store.clone()));
            }
        }
    }

    biggest.unwrap()
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(String, Employee)> {
    let mut highest_salary = 0.0;
    let mut result = vec![];

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for (name, employee) in &store.employees {
                if employee.salary > highest_salary {
                    highest_salary = employee.salary;
                    result = vec![(name.clone(), *employee)];
                } else if (employee.salary - highest_salary).abs() < f64::EPSILON {
                    result.push((name.clone(), *employee));
                }
            }
        }
    }

    result
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut count = 0;

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            count += store.employees.len();
        }
    }

    count
}

pub fn check_for_securities(mall: &mut Mall, mut guard_pool: HashMap<String, Guard>) {
    let total_size: u64 = mall
        .floors
        .values()
        .map(|floor| floor.stores.values().map(|s| s.square_meters).sum::<u64>())
        .sum();

    let required = (total_size + 199) / 200;

    for (name, guard) in guard_pool.drain() {
        if mall.guards.len() >= required as usize {
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
