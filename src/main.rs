mod check_connections;
mod time_check;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::{fs, thread};
use crate::check_connections::check_connections::check_connections::ThreadPool;
use crate::check_connections::connection_handler::connection_handler::handle_connection;
use crate::time_check::get_time::time_check::get_time;
use std::str;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1123").unwrap();
    let pool = ThreadPool::new(100);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream, get_time());
        });
    }
}
