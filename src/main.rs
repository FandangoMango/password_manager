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
        Command::Add(key_args) => vault.add_key(key_args).unwrap_or_else(quit),
        Command::Update(key_args) => vault.update_key(key_args).unwrap_or_else(quit)
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
        let add_arguments = KeyArgs {
            name: "netflix".into(),
            username: Some("Sneaky_peedo".into()),
            url: Some("netflix.com".into()),
            email: Some("Speedos@gmail.com".into()),
            password: Some("password".into()),
        };
        let result = vault.add_key(add_arguments.clone());
        assert!(result.is_ok());
        let result = vault.add_key(add_arguments);
        assert!(result.is_err());
    }
}