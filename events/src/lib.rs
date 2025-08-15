use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let color = format!(
            "\x1b[38;2;{};{};{}m",
            self.color.0, self.color.1, self.color.2
        );
        let reset = "\x1b[0m";
        write!(
            f,
            "{}{} (Size: {}, Position: {:?}){}",
            color, self.content, self.size, self.position, reset
        )
    }
}

use Event::*;

impl Event {
    pub fn notify(&self) -> Notification {
        match self {
            Remainder(content) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: content,
            },
            Registration(duration) => {
                let hours = duration.as_hours();
                let minutes = duration.as_minutes() % 60;
                let seconds = duration.as_secs() % 60;
                Notification {
                    size: 30,
                    color: (255, 2, 22),
                    position: Position::Top,
                    content: format!(
                        "You have {hours}H:{minutes}M:{seconds}S left before the registration ends"
                    ),
                }
            }
            Appointment(content) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: content,
            },
            Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}
