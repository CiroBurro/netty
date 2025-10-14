use std::net::SocketAddr;
use thiserror::Error;

/// TCPError enum
/// It handles errors related to the TCP connection
///
/// Variants:
/// - SocketCreationFailure: Occurs when the reverse shell failes to create the socket
/// - ConnectionFailure: Occurs when the connection between the reverse shell and the listener failes
#[derive(Error, Debug)]
pub enum TCPError {
	#[error("Failed to create socket")]
	SocketCreationFailure(#[from] anyhow::Error),

	#[error("Connection failed: could not connect to {0:?}")]
	ConnectionFailure(SocketAddr),
}

/// CLIError enum
/// It handles errors related to cli arguments
///
/// Variants:
/// - EmptyArgument: Occurs when a flag is specified with no content
/// - IncompatibleArgs: Occurs when two incompatible flags are specified together
#[derive(Error, Debug)]
pub enum CLIError {
	#[error("The argument: {arg} cannot be empty if its flag is specified")]
	EmptyArgument { arg: String },

	#[error("To specify an IP address with listener mode enable is not allowed")]
	IncompatibleArgs,
}