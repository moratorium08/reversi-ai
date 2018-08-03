use board;
use interface::client;
use player::player;
use color;


enum Status {
    Wait,
    Player,
    Opponent,
    WaitAck,
    Terminate,
}

struct Game<T: client::Client> {
    client: T,
    status: Status,
    board: board::Board,
    opponent_name: String,
    time: u64,
    color: color::Color
}

impl<T: client::Client> Game<T> {
    /*fn new(c: T, name: &str) -> Result<Game<T>, String> {
        let open_cmd = client::Command::open_cmd(name);

        match c.output_command(open_cmd) {
            Ok(_) => {
                Ok(Game{
                    client: c,
                    status: Status::Wait,
                    board: board::Board::new(),
                    opponent_name: String::new(),
                    time: 0,
                    color: color::Color::black()
                })
            },
            Err(s) => Err(s)
        }
    }*/

    /*fn wait_start(&mut self) {
        match self.client.input_command() {
            Ok(cmd) => {
                match cmd {
                    client::Command::Bye(scores) => {
                        println!("Bye.");
                    }
                    client::Command::Start(color, oname, t) => {
                        let board = board::Board::new();

                        match color {
                            client::Color::Black => {}
                            client::Color::White => {}
                        }
                    }
                }
            }
            Err(reason) => {
                panic!("Failed to load input command: {}", reason);
            }
        }
    }*/
    /*fn main_loop(mut self) -> Result<(), String>{
        loop {
            match status {
                Wait => wait_start(&c),
                Player => player(&)
            };
        }

        Ok(())
    }*/
}




