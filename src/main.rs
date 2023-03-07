mod vault {
    pub mod components;
}

use crate::vault::components::*;
use std::io;
fn main() -> Result<(), String> {
    let mut vault = KeyVault::new("Main");
    'start: loop {
        let args = get_args();
        if args.len() == 0 {
            println!("No arguments provided.");
        }
        if let Some(arg1) = args.get(0) {
            match arg1.as_str() {
                "add" => vault.add_key(&args[1]).unwrap(),
                _ => continue,
            }
        }
        println!("{}", vault);
    }
    Ok(())
}

fn get_args() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    input
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect()
}
