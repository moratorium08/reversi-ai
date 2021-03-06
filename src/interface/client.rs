pub trait Client {
    fn input_command(&mut self) -> Result<Command, String>;
    fn output_command(&mut self, cmd: Command) -> Result<(), String>;
}

#[derive(PartialEq, Eq, Debug)]
pub enum MatchResult {
    Win,
    Lose,
    Tie,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Move {
    Pass,
    GiveUp,
    Mv(String),
}

impl Move {
    pub fn to_string(&self) -> String {
        match *self {
            Move::Pass => "PASS".to_string(),
            Move::GiveUp => "GIVEUP".to_string(),
            Move::Mv(ref s) => s.clone(),
        }
    }
}


#[derive(PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

pub enum Command {
    Open(String),
    End(MatchResult, u8, u8, String),
    Start(Color, String, u64),
    Ack(u64),
    Move(Move),
    Bye(Vec<(String, (i64, u64, u64))>),
    Empty,
}

impl Command {
    pub fn to_string(&self) -> String {
        match *self {
            Command::Open(ref s) => "OPEN ".to_string() + s,
            Command::Move(ref m) => "MOVE ".to_string() + &(m.to_string()),
            _ => panic!("Oops, not implemented...")
        }
    }

    pub fn open_cmd(name: &str) -> Command {
        Command::Open(name.to_string())
    }

    pub fn move_cmd(mv: Move) -> Command {
        Command::Move(mv)
    }
}
