use std::net::SocketAddr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TCPError {
    #[error("Failed to bind address {ip} and port {port}")]
    BindError {
        ip: String,
        port: u16,
    },

    #[error("Failed to create socket")]
    SocketCreationError(#[from] anyhow::Error),

    #[error("Connection failed: could not connect to {0:?}")]
    ConnectionError (SocketAddr),
}
