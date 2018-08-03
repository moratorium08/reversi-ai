use std::io;
use std::io::{BufRead, Write};
use std::net;

use interface::client;
use interface::parser;

struct Client {
    reader: io::BufReader<net::TcpStream>,
    writer: io::BufWriter<net::TcpStream>,
}

struct ClientBuilder {
    host: String,
    port: u16,
}

impl ClientBuilder {
    fn new() -> ClientBuilder {
        ClientBuilder { host: "localhost".to_string(), port: 3000 }
    }

    fn host(&mut self, host: String) -> &mut ClientBuilder {
        self.host = host;
        self
    }

    fn port(&mut self, port: u16) -> &mut ClientBuilder {
        self.port = port;
        self
    }

    fn finalize(self) -> Result<Client, String> {
        let addr = self.host + ":" + &(self.port.to_string());
        match net::TcpStream::connect(addr) {
            Ok(stream) => {
                match stream.try_clone() {
                    Ok(stream2) => {
                        let writer = io::BufWriter::new(stream);
                        let reader = io::BufReader::new(stream2);
                        Ok(Client { reader, writer} )
                    }
                    Err(_) => {
                        Err("Failed to clone stream".to_string())
                    }
                }
            }
            Err(_) => Err("Failed to connect host/addr".to_string()),
        }
    }
}

impl client::Client for Client {
    fn input_command(&mut self) -> Result<client::Command, String> {
        let mut s = String::new();
        self.reader.read_line(&mut s);
        parser::parse(s)
    }

    fn output_command(&mut self, cmd: client::Command) -> Result<(), String> {
        let s = cmd.to_string();
        let buf = s + "\n";
        match self.writer.write(buf.as_bytes()) {
            Ok(_) => Ok(()),
            _ => Err("Failed to send".to_string())
        }
    }
}