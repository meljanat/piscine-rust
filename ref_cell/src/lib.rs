use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: Cell<i32>,
    pub max: i32,
}

impl Tracker {
    pub fn new(max: i32) -> Self {
        Self {
            messages: RefCell::new(Vec::new()),
            value: Cell::new(0),
            max,
        }
    }

    pub fn set_value(&self, value: &Rc<i32>) {
        let count = Rc::strong_count(value) as i32;

        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
        } else if count > (self.max as f32 * 0.7) as i32 {
            let percent = ((count as f32 / self.max as f32) * 100.0) as i32;
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {}% of your quota!",
                percent
            ));
            self.value.set(count);
        } else {
            self.value.set(count);
        }
    }

    pub fn peek(&self, value: &Rc<i32>) {
        let count = Rc::strong_count(value);
        let percent = ((count as f32 / self.max as f32) * 100.0) as i32;
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            percent
        ));
    }
}
