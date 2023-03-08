#![allow(warnings)]
mod vault;
mod parsing;

use std::io;
use std::process::exit;
use crate::vault::keyvault::*;
use crate::vault::keybox::*;
use crate::parsing::args::*;
use clap::Parser;
fn main() {
    let mut vault = KeyVault::new("Main");
    let args = AppArgs::parse();
    match args.command {
        Command::Add(add_arguments) => vault.add_key(&add_arguments).unwrap_or_else(quit)
    }
}

fn quit(reason: String) {
    println!("{}", reason);
}

mod tests {
    use crate::parsing::args::*;
    use crate::vault::keyvault::*;
    use crate::vault::keybox::*;
    use crate::Command::Add;

    #[test]
    fn adding_key_with_duplicate_name_fails() {
        let mut vault = KeyVault::new("Main");
        let add_arguments = AddKeyArgs {
            name: "netflix".into(),
            username: None,
            url: Some("netflix.com".into()),
            email: None,
            password: Some("password".into()),
        };
        let result = vault.add_key(&add_arguments);
        assert!(result.is_ok());
        let result = vault.add_key(&add_arguments);
        assert!(result.is_err());
    }
}
