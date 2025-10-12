use crate::errors::TCPError;
use anyhow::{self, Context};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tokio::io::{stderr, stdin, stdout, BufReader, BufWriter};
use tokio::net::TcpListener;

pub async fn listen(ip: &str, port: u16) -> anyhow::Result<()> {
	let addr: SocketAddr = SocketAddr::new(IpAddr::from_str("0.0.0.0").with_context(|| format!("Failed to convert the string: {ip} to an IP address"))?, port);
	let listener: TcpListener = TcpListener::bind(addr).await?;
	println!("Listening...");

	match listener.accept().await {
		Ok(s) => {
			println!("Connected");
			let stream = s.0;
			let (reader, writer) = stream.into_split();
			let mut socket_stdin = BufWriter::new(writer);
			let mut socket_stdout = BufReader::new(reader);

			let mut stdin = stdin();
			let mut stdout = stdout();
			let mut stderr = stderr();

			let stdin_handle = tokio::spawn(async move {
				tokio::io::copy(&mut stdin, &mut socket_stdin).await.expect("Failed to copy from stdin to the socket");
			});

			let stdout_handle = tokio::spawn(async move {
				tokio::io::copy(&mut socket_stdout, &mut stdout).await.expect("Failed to copy from the socket to stdout");
				tokio::io::copy(&mut socket_stdout, &mut stderr).await.expect("Failed to copy from the socket to stderr");
			});

			let _ = stdin_handle.await;
			let _ = stdout_handle.await;

			Ok(())
		}
		Err(_e) => {
			Err(TCPError::ConnectionFailure(addr).into())
		}
	}
}