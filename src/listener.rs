use crate::errors::TCPError;
use anyhow::{self, Context};
use std::{
	net::{IpAddr, SocketAddr},
	process::exit,
	str::FromStr,
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::{
	io::{stderr, stdin, stdout, BufReader, BufWriter},
	net::TcpListener,
	sync::mpsc,
};

pub async fn listen(ip: &str, port: u16) -> anyhow::Result<()> {
	let addr: SocketAddr = SocketAddr::new(IpAddr::from_str("0.0.0.0").with_context(|| format!("Failed to convert the string: {ip} to an IP address"))?, port);
	let listener: TcpListener = TcpListener::bind(addr).await?;
	println!("Listening...");

	match listener.accept().await {
		Ok(s) => {
			println!("Connected");
			let (tx, mut rx) = mpsc::channel::<()>(1);
			let stream = s.0;
			let (reader, writer) = stream.into_split();
			let mut socket_stdin = BufWriter::new(writer);
			let mut socket_stdout = BufReader::new(reader);

			let stdin = stdin();
			let mut stdout = stdout();
			let mut stderr = stderr();

			let stdin_handle = tokio::spawn(async move {
				let mut bufreader = BufReader::new(stdin);
				let mut line: String = String::new();

				loop {
					line.clear();
					let n = bufreader.read_line(&mut line).await.expect("Failed to read line from stdin (listener)");

					if n == 0 { break; } // EOF

					if line.trim().eq_ignore_ascii_case("exit") {
						let _ = tx.send(()).await;
						break;
					}

					socket_stdin.write_all(line.as_bytes()).await.context("Failed to write stdin to socket").expect("Failed to write stdin to socket");
					socket_stdin.flush().await.context("Failed to flush socket").expect("Failed to flush socket");
				}
			});

			let stdout_handle = tokio::spawn(async move {
				tokio::io::copy(&mut socket_stdout, &mut stdout).await.expect("Failed to copy from the socket to stdout");
				tokio::io::copy(&mut socket_stdout, &mut stderr).await.expect("Failed to copy from the socket to stderr");
			});


			tokio::select! {
				_ = stdin_handle => {},
				_ = stdout_handle => {},
				_ = rx.recv() => {
					exit(0);
				}
			}

			Ok(())
		}
		Err(_e) => {
			Err(TCPError::ConnectionFailure(addr).into())
		}
	}
}