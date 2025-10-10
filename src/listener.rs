use crate::errors::TCPError;
use anyhow::{self, Context};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tokio::net::TcpListener;

pub async fn listen(ip: &str, port: u16) -> anyhow::Result<()> {
	let addr: SocketAddr = SocketAddr::new(IpAddr::from_str("0.0.0.0").with_context(|| format!("Failed to convert the string: {ip} to an IP address"))?, port);
	let listener: TcpListener = TcpListener::bind(addr).await?;
	println!("Listening...");

	match listener.accept().await {
		Ok(_s) => {
			println!("Connected");
			todo!()
		}
		Err(_e) => Err(TCPError::ConnectionFailure(addr).into())
	}
}