use board;
use interface::client;
use player::player;
use pmove;
use color;


enum Status {
    Wait,
    Player,
    Opponent,
    Terminate,
}

pub struct Game<T: client::Client, S: player::Player> {
    client: T,
    status: Status,
    color: color::Color,
    board: board::Board,
    opponent_name: String,
    time: u64,
    player: S,
}

impl<T: client::Client, S: player::Player> Game<T, S> {
    pub fn new(mut c: T, pl: S, name: &str) -> Result<Game<T, S>, String> {
        let open_cmd = client::Command::open_cmd(name);

        match (&mut c).output_command(open_cmd) {
            Ok(_) => Ok(Game {
                client: c,
                status: Status::Wait,
                board: board::Board::new(),
                opponent_name: String::new(),
                time: 0,
                color: color::Color::black(),
                player: pl,
            }),
            Err(s) => Err(s)
        }
    }

    fn wait_start(&mut self) {
        let c = &mut self.client;
        match c.input_command() {
            Ok(cmd) => {
                match cmd {
                    client::Command::Bye(scores) => {
                        self.status = Status::Terminate;
                        println!("Bye.");
                    }
                    client::Command::Start(color, oname, t) => {
                        self.opponent_name = oname;
                        self.board = board::Board::new();

                        match color {
                            client::Color::Black => {
                                self.color = color::Color::black();
                                self.status = Status::Player;
                            }
                            client::Color::White => {
                                self.color = color::Color::white();
                                self.status = Status::Opponent;
                            }
                        }
                    }
                    x => panic!("Invalid Command")
                }
            }
            Err(reason) => {
                panic!("Failed to load input command: {}", reason);
            }
        }
    }

    fn do_move(mv: &pmove::Move, board: board::Board, color: color::Color) -> board::Board {
        match *mv {
            pmove::Move::Mv(ref p) => {
                board.flip(p, color)
            }
            _ => board
        }
    }

    fn pmove2cmove(mv: pmove::Move) -> client::Move {
        match mv {
            pmove::Move::Mv(p) => {
                let s = p.to_string();
                client::Move::Mv(s)
            }
            pmove::Move::Pass => client::Move::Pass,
            pmove::Move::GiveUp => client::Move::GiveUp
        }
    }

    fn cmove2pmove(mv: client::Move) -> pmove::Move {
        match mv {
            client::Move::Mv(s) => {
                match board::Pos::from_str(s) {
                    Ok(p) => pmove::Move::Mv(p),
                    Err(s) => panic!("Illegal move command: {}", s)
                }
            }
            client::Move::Pass => pmove::Move::Pass,
            client::Move::GiveUp => pmove::Move::GiveUp
        }
    }

    fn my_move(&mut self) {
        let c = &mut self.client;
        let pmove = self.player.play(self.board, self.color);
        self.board = Game::<T, S>::do_move(&pmove, self.board, self.color);

        let cmove = Game::<T, S>::pmove2cmove(pmove);
        match c.output_command(client::Command::Move(cmove)) {
            Ok(_) => {
                self.board.print();
            }
            Err(s) => {
                panic!("Failed to send command: {}", s);
            }
        }

        match c.input_command() {
            Ok(cmd) => {
                match cmd {
                    client::Command::Ack(time) => {
                        self.time = time;
                        self.status = Status::Opponent;
                    }
                    client::Command::End(result, n, m, r) => {
                        self.status = Status::Wait;
                    }
                    x => panic!("Invalid Command")
                }
            }
            Err(s) => panic!("Failed to input command: {}", s)
        }
    }

    fn op_move(&mut self) {
        let c = &mut self.client;
        match c.input_command() {
            Ok(cmd) => {
                match cmd {
                    client::Command::Move(cmove) => {
                        let pmove = Game::<T, S>::cmove2pmove(cmove);
                        self.board = Game::<T, S>::do_move(&pmove, self.board, self.color.opposite());
                        self.status = Status::Player;
                    }
                    client::Command::End(result, n, m, r) => {
                        self.status = Status::Wait;
                    }
                    x => panic!("Invalid Command")
                }
            }
            Err(s) => panic!("Failed to input command: {}", s)
        }
    }

    fn proc_end(&self, result: client::MatchResult, n: u8, m: u8, reason: String) {}


    pub fn main_loop(mut self) -> Result<(), String> {
        loop {
            match self.status {
                Status::Wait => self.wait_start(),
                Status::Player => self.my_move(),
                Status::Opponent => self.op_move(),
                Status::Terminate => break,
            };
        }
        Ok(())
    }
}
