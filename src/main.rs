use std::{process, sync::{Arc, Mutex}, thread};
use peer::*;

/// The entry point of the application. Initializes the node and starts two threads: 
/// one for listening to incoming connections / gossip 
/// and another for gossiping messages to peers.
fn main() {
    let peers: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let peers1 = Arc::clone(&peers);
    let peers2 = Arc::clone(&peers);

    let (socket, period) = node_init::init_node(peers)
        .unwrap_or_else(|err| {
            println!("{:?}", err.to_string());
            process::exit(1);
        });

    let socket1 = Arc::new(Mutex::new(socket.try_clone().unwrap()));
    let socket2 = Arc::new(Mutex::new(socket.try_clone().unwrap()));

    let t1 = thread::spawn(move || listener::listen(peers1, socket1));
    let t2 = thread::spawn(move || gossiper::gossip(peers2, socket2, period));
    
    t1.join().unwrap();
    t2.join().unwrap();
}
