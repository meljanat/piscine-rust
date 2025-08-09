use std::cell::RefCell;
use std::rc::Rc;

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    pub value: i32,
    pub max: i32,
}

impl Tracker {
    pub fn new(max: i32) -> Self {
        Self {
            messages: RefCell::new(Vec::new()),
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: &Rc<i32>) {
        let count = Rc::strong_count(value) as i32;

        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_string());
        } else if count > (self.max as f32 * 0.7) as i32 {
            let percent = ((count as f32 / self.max as f32) * 100.0).round();
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {}% of your quota!",
                percent
            ));
            self.value = count;
        } else {
            self.value = count;
        }
    }

    pub fn peek(&mut self, value: &Rc<i32>) {
        let count = Rc::strong_count(value);
        let percent = ((count as f32 / self.max as f32) * 100.0).round();
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {}% of your quota",
            percent
        ));
    }
}
