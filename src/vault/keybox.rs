use std::fmt;
use crate::parsing::KeyArgs;
#[macro_use]
use crate::vault::keyvault::*;
#[derive(Debug)]
pub struct KeyBox {
    pub name: String,
    url: Option<String>,
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

impl From<KeyArgs> for KeyBox {
    fn from(args: KeyArgs) -> Self {
        let (name, username, url, email, password) = (args.name, args.username, args.url, args.email, args.password);
        KeyBox {
            name, username, url, email, password
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
