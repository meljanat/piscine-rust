#[derive(Debug)]
pub struct Person {
    pub name: &str,
    pub age: u8,
}

impl Person {
    pub fn new(name: &str) -> Person {
        Person { name, age: 0 }
    }
}
