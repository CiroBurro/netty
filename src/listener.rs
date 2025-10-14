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


/// Listen function to run the program in listener mode
/// It listens for a new TCP connection and handles it
///
/// Args:
/// - ip: ip address to connect to
/// - port: port to bind to the ip address
pub async fn listen(ip: &str, port: u16) -> anyhow::Result<()> {
	let addr: SocketAddr = SocketAddr::new(IpAddr::from_str("0.0.0.0").with_context(|| format!("Failed to convert the string: {ip} to an IP address"))?, port);
	let listener: TcpListener = TcpListener::bind(addr).await?;
	println!("Listening...");

	// Wait for a new connection
	match listener.accept().await {
		Ok(s) => {
			println!("Connected to {}", addr);

			let (tx, mut rx) = mpsc::channel::<()>(1); // Needed to transimit the exit flag
			let stream = s.0;

			// Defines standard input and standard output of the stream
			let (reader, writer) = stream.into_split();
			let mut socket_stdin = BufWriter::new(writer);
			let mut socket_stdout = BufReader::new(reader);

			// Defines standard input and standard output of the process
			let stdin = stdin();
			let mut stdout = stdout();
			let mut stderr = stderr();

			// Async task to handle stdin reading/writing
			let stdin_handle = tokio::spawn(async move {
				// Can't simply use tokio::io::copy because the need of reading the exit command and close the process
				// Use a bufreader instead
				let mut bufreader = BufReader::new(stdin);
				let mut line: String = String::new();

				loop {
					// Read commands from stdin in loop until EOF or "exit" command
					line.clear();
					let n = bufreader.read_line(&mut line).await.expect("Failed to read line from stdin (listener)");

					if n == 0 { break; } // EOF

					if line.trim().eq_ignore_ascii_case("exit") {
						let _ = tx.send(()).await;
						break;
					}

					// Write the commands to the standard input of the stream
					socket_stdin.write_all(line.as_bytes()).await.context("Failed to write stdin to socket").expect("Failed to write stdin to socket");
					socket_stdin.flush().await.context("Failed to flush socket").expect("Failed to flush socket");
				}
			});

			// Async task to handle stdout reading/writing
			let stdout_handle = tokio::spawn(async move {
				// Copy from the standard output of the stream to the standard output and error of the process
				tokio::io::copy(&mut socket_stdout, &mut stdout).await.expect("Failed to copy from the socket to stdout");
				tokio::io::copy(&mut socket_stdout, &mut stderr).await.expect("Failed to copy from the socket to stderr");
			});


			// Wait for the first task to finish (exit command)
			tokio::select! {
				_ = stdin_handle => {},
				_ = stdout_handle => {},
				_ = rx.recv() => {
					println!("Connection closed successfully");
					exit(0);
				}
			}

			Ok(())
		}
		Err(_e) => {
			// If connection failes returns a ConnectionFailur error
			Err(TCPError::ConnectionFailure(addr).into())
		}
	}
}