mod check_connections;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, thread};
use crate::check_connections::check_connections::check_connections::ThreadPool;
use crate::check_connections::connection_handler::connection_handler::handle_connection;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(100);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
