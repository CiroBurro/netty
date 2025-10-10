use std::net::SocketAddr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TCPError {
	#[error("Failed to create socket")]
	SocketCreationFailure(#[from] anyhow::Error),

	#[error("Connection failed: could not connect to {0:?}")]
	ConnectionFailure(SocketAddr),
}

#[derive(Error, Debug)]
pub enum CLIError {
	#[error("The argument: {arg} cannot be empty if its flag is specified")]
	EmptyArgument { arg: String },

	#[error("To specify an IP address with listener mode enable is not allowed")]
	IncompatibleArgs,
}