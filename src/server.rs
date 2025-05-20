use std::io::Read;
use std::net::TcpListener;
use crate::http::request::Request;
use std::convert::TryFrom;
pub struct Server {
    host: String,
}

impl Server {
    pub fn new(host_data: String) -> Server {
        return Server {
            host: host_data,
        };
    }

    pub fn run(&self) {
        let tcp_result = TcpListener::bind(self.host.clone());
        match tcp_result {
            Ok(tcp) => {
                println!("Started connection in address {}", self.host);                        
                let new_connection_stream = tcp.accept();
                match new_connection_stream {
                    Ok((mut stream, addr)) => {
                        println!("New client from {} connected.", addr);
                        let mut buff = [0; 1024];
                        let data_result = stream.read(&mut buff);
                        match data_result {
                            Ok(_) => {
                                // Capture the Result from `try_from`
                                match Request::try_from(&buff[..]) {
                                    Ok(_) => {
                                        // Handle the successfully parsed request
                                        println!("Parsed request with path");
                                    },
                                    Err(e) => {
                                        // Handle the error when parsing the request
                                        println!("Error parsing request: {}", e);
                                    },
                                }
                            },
                            Err(_) => println!("Problem reading from the client {}", addr),
                        }
                    },
                    Err(_) => println!("Error! Problem accepting new connections!"),
                }
            },
            Err(_) => {
                println!("Unable to establish connection at address {}", self.host);
            }
        }
    }
}
