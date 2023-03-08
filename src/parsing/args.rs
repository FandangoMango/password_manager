use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about = "A CLI for safely(-ish) storing passwords")]
pub struct AppArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Add a new keybox to the vault
    #[command(arg_required_else_help = false)]
    Add(AddKeyArgs),
    /*/// Display contents in a keybox
    Show(ShowKey),
    /// Update an existing keybox
    Update(UpdateKey),
    /// Delete a keybox
    Delete(DeleteKey), */
}
#[derive(Debug, Args)]
pub struct AddKeyArgs {
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
