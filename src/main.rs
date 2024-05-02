use bleebo::{server, users::insert_new_user};
use clap::{Parser, Subcommand};

/// Bleebo
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[command(subcommand)]
    command: Commands,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Subcommand, Debug)]
enum Commands {
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
        Commands::Start => {
            server::build().launch().await?;
        }
        Commands::NewUser { username } => match insert_new_user(username) {
            Ok(password) => println!("Added user {username} with temporary password {password}"),
            Err(e) => println!("Failed to add user with error: {}", e),
        },
    }

    Ok(())
}
