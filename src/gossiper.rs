use std::{net::UdpSocket, sync::{Arc, Mutex}, time::SystemTime};

/// Periodically sends gossip messages to a list of peers over a UDP socket.
///
/// # Arguments
///
/// * `peers` - A thread-safe, shared vector of peer addresses.
/// * `socket` - A thread-safe, shared UDP socket used to send messages.
/// * `period` - The interval (in seconds) at which gossip messages are sent.
pub fn gossip(peers: Arc<Mutex<Vec<String>>>, socket: Arc<Mutex<UdpSocket>>, period: u64){
    let mut last_gossiped = SystemTime::now();
    loop {
        if let Ok(duration) = last_gossiped.elapsed() {
            if duration.as_secs() >= period {
                last_gossiped = SystemTime::now();

                let peers = peers.lock().unwrap();
                if !peers.is_empty(){
                    println!("Sending message [Hello!] to {:?}", peers);
                }

                for peer in peers.iter(){
                    let buf = "GOSPHello!".as_bytes();
                    let socket = socket.lock().unwrap();

                    match socket.send_to(buf, peer) {
                        Ok(_) => (),
                        Err(err) => println!("Error sending to peer {peer}: {:?}", err),
                    };
                }
            }
        }
    }
}