use std::io;
use std::net;

use interface::client;
use interface::parser;

struct Client {
    reader: io::BufReader<net::TcpStream>,
    writer: io::BufWriter<net::TcpStream>,
    name: String,
}

struct ClientBuilder {
    host: String,
    port: u16,
    name: String,
}

impl ClientBuilder {
    fn new() -> ClientBuilder {
        ClientBuilder { host: "localhost".to_string(), port: 3000, name: "reversi".to_string() }
    }

    fn host(&mut self, host: String) -> &mut ClientBuilder {
        self.host = host;
        self
    }

    fn port(&mut self, port: u16) -> &mut ClientBuilder {
        self.port = port;
        self
    }

    fn name(&mut self, name: String) -> &mut ClientBuilder {
        self.name = name;
        self
    }

    fn finalize(self) -> Result<Client, String> {
        let addr = self.host + ":" + self.port.to_string();
        match net::TcpStream::connect(addr) {
            Ok(stream) => {
                match stream.try_clone() {
                    Ok(stream2) => {
                        let writer = io::BufWriter::new(stream);
                        let reader = io::BufReader::new(stream);
                        Client { reader, writer, self.name }
                    },
                    Err(_) => {
                        Err("Failed to clone stream".to_string())
                    }
                }
            },
            Err(_) => Err("Failed to connect host/addr".to_string()),
        }
    }
}

impl client::Client for Client {
    fn input_command(&self) -> Result<Command, String> {
        let mut s = String::new();
        self.reader.read_line(&s);
        parser::parse(s)
    }

    fn output_command(&self, cmd: Command) -> Result<(), String> {
        let s = cmd.to_string();
        match self.writer.write(s + "\n") {
            Ok(_) => Ok(()),
            _ => Err("Failed to send")
        }
    }

    fn name(&self) -> &str{
        &self.name
    }
}