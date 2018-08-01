use interface::client;

struct Client {
    host: String,
    port: u16,
    name: String,
}

struct ClientBuilder {
    host: String,
    port: u16,
    name: String,
}

impl ClientBuilder {
    fn new() -> ClientBuilder {
        ClientBuilder("localhost".to_string(), 3000, "reversi".to_string())
    }

    fn host(&mut self, host: String) {
        self.host = host;
        self
    }

    fn port(&mut self, port: u16) {
        self.port = port;
        self
    }

    fn name(&mut self, name: String) {
        self.name = name;
        self
    }

    fn finalize(&self) -> Client {
        Client{host: self.host, port: self.port, name: self.name}
    }
}

impl client::Client for Client {
    fn game_loop(&self) {

    }

    fn name(&self) {
        self.name
    }
}