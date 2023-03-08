use std::fmt;
#[macro_use]
use crate::vault::keyvault::*;
#[derive(Debug)]
pub struct KeyBox {
    name: String,
    url: Option<String>,
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}
impl KeyBox {
    pub fn new(name: &str) -> KeyBox {
        KeyBox {
            name: name.into(),
            url: None,
            username: None,
            email: None,
            password: None,
        }
    }
}

#[macro_export]
macro_rules! display_option {
    ($name:expr, $value:expr, $f:expr) => {
        if let Some(value) = $value {
            writeln!($f, r#"      {}: {},"#, $name, value)?;
        }
    };
}

impl fmt::Display for KeyBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  {}: {{", self.name)?;
        display_option!(r#"Name"#, Some(&self.name), f);
        display_option!(r#"Username"#, &self.username, f);
        display_option!(r#"Email"#, &self.email, f);
        display_option!(r#"Password"#, &self.password, f);
        writeln!(f, "    }}")?;
        Ok(())
    }
}
