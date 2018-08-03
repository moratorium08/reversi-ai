use board;

pub enum Move {
    Mv(board::Pos),
    Giveup,
    Pass,
}