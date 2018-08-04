use std::io;
use std::io::{BufRead, Write};
use std::net;

use interface::client;
use interface::parser;

pub struct Client {
    reader: io::BufReader<net::TcpStream>,
    writer: io::BufWriter<net::TcpStream>,
}

pub struct ClientBuilder<'a> {
    host: &'a str,
    port: u16,
}

impl<'a> ClientBuilder<'a> {
    pub fn new() -> ClientBuilder<'a> {
        ClientBuilder { host: "localhost", port: 3000 }
    }

    pub fn host(&'a mut self, host: &'a str) -> &'a mut ClientBuilder {
        self.host = host;
        self
    }

    pub fn port(&'a mut self, port: u16) -> &'a mut ClientBuilder {
        self.port = port;
        self
    }

    pub fn finalize(&self) -> Result<Client, String> {
        let addr = self.host.to_string() + ":" + &(self.port.to_string());
        match net::TcpStream::connect(addr) {
            Ok(stream) => {
                match stream.try_clone() {
                    Ok(stream2) => {
                        let writer = io::BufWriter::new(stream2);
                        let reader = io::BufReader::new(stream);
                        Ok(Client { reader, writer })
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
        let mut ret: Result<client::Command, String>;
        loop {
            let mut s = String::new();
            self.reader.read_line(&mut s);
            if s.len() == 0 {
                continue;
            }

            println!("Received: {}", &s);
            let tmp = parser::parse(s);
            match tmp {
                Ok(client::Command::Empty) => (),
                x => {
                    ret = x;
                    break;
                }
            }
        }
        ret
    }

    fn output_command(&mut self, cmd: client::Command) -> Result<(), String> {
        let buf = cmd.to_string();
        println!("Sent: {}", &buf);
        match writeln!(self.writer, "{}", buf) {
            Ok(_) => {
                self.writer.flush();
                Ok(())
            }
            _ => Err("Failed to send".to_string())
        }
    }
}