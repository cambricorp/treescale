#![allow(dead_code)]
extern crate mio;

use self::mio::Token;
use self::mio::channel::{Sender};
use self::mio::tcp::TcpStream;
use network::tcp::TcpWriterCommand;

pub enum ConnectionType {
    TCP
}

/// Base Connection structure for handling base information of connection
pub struct Connection {
    pub value: u64,
    pub socket_token: Token,
    pub api_version: usize,
    pub conn_type: ConnectionType,
    pub from_server: bool,

    // writer command for TCP connection or None if this is not a TCP connection
    tcp_writer_chan: Option<Sender<TcpWriterCommand>>,
    // keeping TCP socket here or None if this is not a TCP connection
    tcp_socket: Option<TcpStream>
}
