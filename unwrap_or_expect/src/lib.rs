#[derive(PartialEq)]
pub enum Security {
    Unknown,
    Message,
    Warning,
    NotFound,
    UnexpectedUrl,
}

pub fn fetch_data(server: Result<&str, &str>, security_level: Security) -> String {
    match server {
        Ok(data) => {
            if security_level == Security::UnexpectedUrl {
                panic!("{}", data);
            }
            data.to_string()
        }
        Err(err) => match security_level {
            Security::Unknown => {
                panic!();
            }
            Security::Message => {
                panic!("ERROR: program stops");
            }
            Security::Warning => "WARNING: check the server".to_string(),
            Security::NotFound => format!("Not found: {}", err),
            Security::UnexpectedUrl => err.to_string(),
        },
    }
}
