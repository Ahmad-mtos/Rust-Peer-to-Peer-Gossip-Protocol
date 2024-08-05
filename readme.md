# Rust Peer-to-Peer Gossip Protocol

This project implements a peer-to-peer gossip protocol using Rust. It provides functionalities for initializing nodes, listening for incoming messages, and gossiping messages to peers. The network operates as a full mesh where each node communicates with all other nodes.

## Table of Contents

- [Overview](#overview)
- [Installation](#installation)
- [Usage](#usage)
- [Protocols](#protocols)

## Overview

This project demonstrates a simple implementation of a peer-to-peer network where nodes communicate with each other using a gossip protocol. Nodes can be initialized with a specific port and optionally connect to an initial peer. The network is fully meshed, meaning its fully connected and each node can communicate with all other nodes.

## Installation

To install and run this project, you need to have Rust and Cargo installed. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/).

Clone the repository:

```sh
git clone https://github.com/Ahmad-mtos/Rust-Peer-to-Peer-Gossip-Protocol
cd Rust-Peer-to-Peer-Gossip-Protocol
```

Build the project:

```sh
cargo build --release
```

## Usage

Navigate to the target release directory:

```sh
cd target/release
```

Run the application with the following command:

```sh
./peer --period <gossip_period> --port <port> [--connect <peer_address>]
```

* `--period`: The interval (in seconds) at which gossip messages are sent (must be greater than 0).

* `--port`: The port number to bind the UDP socket.

* `--connect`: Optional. The address of an initial peer to connect to.

Example:

```sh
./peer --period 5 --port 8080 --connect 127.0.0.1:8081
```

## Protocols

The project uses the following protocols for communication between nodes:

- **INIT**: When a new node joins the network, it sends an INIT protocol message to the initial peer. This message signals the new node's attempt to join the network.

- **INFO**: The initial peer responds to an INIT message with an INFO protocol message. This message contains a list of peer addresses from the initial peer's network.

- **CONN**: After receiving the INFO message, the new node sends a CONN protocol message to all peers in the network. This message notifies existing peers of the new node's presence, prompting them to add the new node to their peer list.

- **GOSP**: Periodically, each node sends a GOSP protocol message to all its peers. This message contains information for gossiping purposes and ensures that all nodes are updated with the latest information.
