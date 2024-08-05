use std::{net::UdpSocket, sync::{Arc, Mutex}};

use crate::{protocol, BUFFER_SIZE};

/// Listens for incoming UDP messages and handles them based on the protocol type.
///
/// This function runs in a loop, receiving messages from the UDP socket and processing them.
/// It handles different protocols such as INIT, CONN, and GOSP, updating the peer list and
/// printing received messages as necessary.
///
/// # Arguments
///
/// * `peers` - A thread-safe, shared vector of peer addresses.
/// * `socket` - A thread-safe, shared UDP socket used to receive messages.
pub fn listen(peers: Arc<Mutex<Vec<String>>>, socket: Arc<Mutex<UdpSocket>>) {
    loop {
        let mut buf = [0; BUFFER_SIZE];
        let socket = socket.lock().unwrap();

        let (number_of_bytes, src_addr) = match socket.recv_from(&mut buf) {
            Ok(res) => res,
            Err(e) => {println!("Error while listening: {:?}", e); continue;},
        };

        let mut peers = peers.lock().unwrap();
        let ip_addr = src_addr.ip().to_string();
        let addr = format!("{ip_addr}:{}", src_addr.port());

        let (protocol, msg) = protocol::parse_buffer(&mut buf, number_of_bytes);

        if protocol == "INIT" {
            protocol::handle_init_protocol(&mut peers, &socket, addr);
        }
        else if protocol == "CONN" {
            protocol::handle_conn_protocol(&mut peers, addr);
        }
        else if protocol == "GOSP" {
            protocol::handle_gosp_protocol(msg, &addr)
        } 
        else {
            println!("Unrecognized protocol received: {protocol}.");
        }
    }
}