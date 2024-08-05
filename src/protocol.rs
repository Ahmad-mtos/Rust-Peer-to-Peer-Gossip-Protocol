use std::{net::UdpSocket, str, sync::MutexGuard};

/// Parses a buffer to extract the protocol identifier and message.
///
/// # Arguments
///
/// * `buf` - A mutable reference to the buffer containing the data.
/// * `buffer_size` - The size of the buffer.
///
/// # Returns
///
/// * `(String, String)` - A tuple containing the protocol identifier and the message as strings.
pub fn parse_buffer(buf: &mut [u8], buffer_size: usize) -> (String, String) {
    let protocol: [u8; 4] = buf[0..4].try_into().unwrap();
    let protocol = str::from_utf8(&protocol).unwrap();

    let msg = str::from_utf8(buf[4..buffer_size].try_into().unwrap()).unwrap().trim_matches(char::from(0));

    (protocol.to_string(), msg.to_string())
}

/// Handles the GOSP protocol by printing the received message and the address of the sender.
///
/// # Arguments
///
/// * `msg` - The received message as a string.
/// * `addr` - The address of the sender.
pub fn handle_gosp_protocol(msg: String, addr: &String) {
    println!("Received message [{msg}] from {addr}");
}

/// Handles the INFO protocol by sending a connection message to new peers and adding them to the peer list.
///
/// # Arguments
///
/// * `peers` - A mutable reference to a MutexGuard containing the list of peers.
/// * `socket` - The UDP socket used to send messages.
/// * `msg` - The message containing new peer addresses separated by spaces.
pub fn handle_info_protocol(peers: &mut MutexGuard<Vec<String>>, socket: &UdpSocket, msg: String) {
    let new_peers: Vec<&str> = msg.split(' ').collect();
    for peer in new_peers {
        if !peer.is_empty() {
            let buf = "CONN".as_bytes();
            match socket.send_to(buf, peer) {
                Ok(_) => peers.push(peer.to_string()),
                Err(err) => println!("Error sending to peer {peer}: {:?}", err),
            };
        }
    }
}

/// Handles the INIT protocol by informing the new peer of the existing peers and adding the new peer to the list.
///
/// # Arguments
///
/// * `peers` - A mutable reference to a MutexGuard containing the list of peers.
/// * `socket` - A MutexGuard containing the UDP socket used to send messages.
/// * `addr` - The address of the new peer.
pub fn handle_init_protocol(peers: &mut MutexGuard<Vec<String>>, socket: &MutexGuard<UdpSocket>, addr: String) {
    println!("New peer joined with address \"{}\"", addr);
    let msg = "INFO".to_owned() + &peers.join(" ");
    let buf = msg.as_bytes();
    match socket.send_to(buf, addr.clone()) {
        Ok(_) => peers.push(addr),
        Err(err) => println!("Error sending to peer {addr}: {:?}", err),
    };
}

/// Handles the CONN protocol by adding a new peer to the list.
///
/// # Arguments
///
/// * `peers` - A mutable reference to a MutexGuard containing the list of peers.
/// * `addr` - The address of the new peer.
pub fn handle_conn_protocol(peers: &mut MutexGuard<Vec<String>>, addr: String) {
    println!("New peer joined with address \"{}\"", addr);
    peers.push(addr);
}