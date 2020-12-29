pub mod connection_handler{
    use std::net::TcpStream;
    use std::io::{Read, Write};
    use std::fs;

    pub fn handle_connection(mut stream: TcpStream, time: String) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        //let contents = fs::read_to_string("html_templates/hello.html").unwrap();
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