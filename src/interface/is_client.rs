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

    fn finalize(self) -> Client {
        Client { host: self.host, port: self.port, name: self.name }
    }
}

impl client::Client for Client {
    fn game_loop(&self) {}

    fn name(&self) -> &String {
        &self.name
    }
}