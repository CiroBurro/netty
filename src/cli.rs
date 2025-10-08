use clap::Parser;

/// Basic reverse shell over TCP with listener and payload
#[derive(Parser, Debug)]
#[command(author="CiroBurro", name = "TCP Reverse Shell", about = "POC reverse shell written in rust", long_about = None)]
pub struct Args {
    /// IP address to connect to
    #[arg(short, long, default_value = "127.0.0.1")]
    pub address: String,

    /// Port to connect to
    #[arg(short, long, default_value = "8080")]
    pub port: u16,

    /// Enable listener mode
    #[arg(short, long)]
    pub listen: bool

}