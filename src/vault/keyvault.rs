use crate::vault::*;
use crate::AddKeyArgs;

use std::collections::HashMap;
use std::fmt;
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

    pub fn add_key(&mut self, args: &AddKeyArgs) -> Result<(), String> {
        let name = &args.name;
        if self.keys.contains_key(name) {
            return Err(format!("Fatal: Key with name '{}' already exists.\n\n\
            Tip: Use another name for the key or delete the existing one", name));
        }
        let key_box = KeyBox::new(name);
        self.keys.insert(name.to_string(), key_box);
        Ok(())
    }
}

impl fmt::Display for KeyVault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Vault: {}", self.vault_name)?;
        writeln!(f, "  Keys in {}:", self.vault_name)?;
        for (_, key_box) in &self.keys {
            writeln!(f, "  {}", key_box)?;
        }
        Ok(())
    }
}
