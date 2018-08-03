pub trait Client {
    fn input_command(&self) -> Result<Command, String>;
    fn output_command(&self, cmd: Command) -> Result<(), String>;
    fn name(&self) -> &str;
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
    Bye(Vec<(String, (u64, u64, u64))>),
    Empty,
}
