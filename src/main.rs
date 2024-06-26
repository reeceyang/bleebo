use bleebo::{
    client::{change_password, delete, upload},
    server,
    users::insert_new_user,
};
use clap::{Parser, Subcommand};

/// bleebo
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run server commands
    Server {
        #[command(subcommand)]
        command: ServerCommands,
    },

    /// Change your password
    ChangePassword,

    /// Upload the current directory
    Upload { site_name: String },

    /// Delete a site
    Delete { site_name: String },
}

#[derive(Subcommand, Debug)]
enum ServerCommands {
    /// Start the server
    Start,

    /// Insert a new user
    NewUser { username: String },
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Server { command } => match command {
            ServerCommands::Start => {
                server::build().launch().await?;
            }
            ServerCommands::NewUser { username } => match insert_new_user(username) {
                Ok(password) => {
                    println!("Added user {username} with temporary password {password}")
                }
                Err(e) => println!("Failed to add user with error: {}", e),
            },
        },
        Commands::ChangePassword => change_password(),
        Commands::Upload { site_name } => upload(&site_name),
        Commands::Delete { site_name } => delete(&site_name),
    }

    Ok(())
}
