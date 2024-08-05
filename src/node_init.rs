use std::{io::{Error, ErrorKind}, net::UdpSocket, sync::{Arc, Mutex}};
use clap::Parser;

use crate::{protocol, BUFFER_SIZE};

/// Command-line arguments parser using Clap.
#[derive(Parser, Debug)]
struct Cli {
    #[arg(long)]
    period: u64,

    #[arg(long)]
    port: u16,

    #[arg(long)]
    connect: Option<String>
}

/// Initializes a node with a UDP socket and a list of peers.
///
/// This function parses command-line arguments to set the gossip period and port,
/// binds a UDP socket to the specified port, and optionally connects to an initial peer.
/// If a connection address is provided, it sends an initialization message and waits for
/// a response to handle the INFO protocol.
///
/// # Arguments
///
/// * `peers` - A thread-safe, shared vector of peer addresses.
///
/// # Returns
///
/// * `Result<(UdpSocket, u64)>` - A tuple containing the bound UDP socket and the gossip period.
///
/// # Errors
///
/// * Returns an error if the period is less than 1 second.
/// * Returns an error if the socket binding fails or if unexpected protocol messages are received.
pub fn init_node(peers: Arc<Mutex<Vec<String>>>) -> std::io::Result<(UdpSocket, u64)> {
    let args = Cli::parse();
    if args.period < 1 {
        return Err(Error::new(ErrorKind::Other, "Period should be more than 0 seconds."))
    }
    let addr = format!("127.0.0.1:{}", args.port);
    println!("My address is \"{addr}\"");
    let socket = UdpSocket::bind(addr)?;
    
    if let Some(connection) = args.connect {
        let mut peers = peers.lock().unwrap();
        let addr = connection.clone();
        peers.push(addr.clone());

        let buf = "INIT".as_bytes();
        socket.send_to(buf, addr)?;

        let mut buf = [0; BUFFER_SIZE];
        socket.recv_from(&mut buf)?;

        let (protocol, msg) = protocol::parse_buffer(&mut buf, BUFFER_SIZE);
        if protocol != "INFO" {
            return Err(Error::new(ErrorKind::Other, "Unexpected protocol received: Expected INFO protocol, got {protocol} instead.")) 
        }
        protocol::handle_info_protocol(&mut peers, &socket, msg);
        println!("Connected to peers at: {:?}", peers);
    }
    Ok((socket, args.period))
}
