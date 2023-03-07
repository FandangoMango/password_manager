use std::collections::HashMap;
use std::fmt;
#[derive(Debug)]
pub struct KeyBox {
    name: String,
    url: Option<String>,
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}
#[derive(Debug)]
pub struct KeyVault {
    vault_name: String,
    keys: HashMap<String, KeyBox>,
}
impl KeyVault {
    pub fn new(vault_name: &str) -> KeyVault {
        KeyVault {
            vault_name: vault_name.into(),
            keys: HashMap::new(),
        }
    }

    pub fn add_key(&mut self, name: &str) -> Result<(), String> {
        if self.keys.contains_key(name) {
            return Err("key with that name already exists".into());
        }
        let key_box = KeyBox::new(name);
        self.keys.insert(name.to_string(), key_box);
        Ok(())
    }
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

macro_rules! display_option {
    ($name:expr, $value:expr, $f:expr) => {
        if let Some(value) = $value {
            writeln!($f, r#"  {}: {},"#, $name, value)?;
        }
    };
}

impl fmt::Display for KeyBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}: {{", self.name)?;
        writeln!(f, r#"  Name: {},"#, self.name)?;
        display_option!(r#"Username"#, &self.username, f);
        display_option!(r#"Email"#, &self.email, f);
        display_option!(r#"Password"#, &self.password, f);
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl fmt::Display for KeyVault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Vault: {}", self.vault_name)?;
        writeln!(f, "Keys in {}:", self.vault_name)?;
        for (_, key_box) in &self.keys {
            writeln!(f, "  {}", key_box)?;
        }
        Ok(())
    }
}