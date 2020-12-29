//! This module handles the response for each request aimed at the server. It includes
//! just one function: handle_connection.
pub mod connection_handler{
    use std::net::TcpStream;
    use std::io::{Read, Write};
    use std::fs;


    /// This function gets a TcpStream and a string that represents current time. It
    /// writes the response in HTTP 1.1 format.
    pub fn handle_connection(mut stream: TcpStream, time: String) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let contents= time;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}