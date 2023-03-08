use clap::{Args, Parser, Subcommand};

use crate::vault::KeyBox;

#[derive(Debug, Parser)]
#[clap(author, version, about = "A CLI for safely(-ish) storing passwords")]
pub struct AppArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a new keybox to the vault
    #[command(arg_required_else_help = true)]
    Add(KeyArgs),
    /// Update an existing keybox
    
    Update(KeyArgs),
    /*/// Display contents in a keybox
    Show(ShowKey),

    /// Delete a keybox
    Delete(DeleteKey), */
}
#[derive(Debug, Args, Clone)]
pub struct KeyArgs {
    /// Key name
    pub name: String,
    
    #[arg(short = 'u', long = "username")]
    pub username: Option<String>,

    #[arg(short = 'l', long = "url")]
    pub url: Option<String>,

    #[arg(short = 'e', long = "email")]
    pub email: Option<String>,

    #[arg(short = 'p', long = "password")]
    pub password: Option<String>,
}