use bleebo::{client::change_password, server, users::insert_new_user};
use clap::{Parser, Subcommand};

/// Bleebo
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
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
    }

    Ok(())
}
