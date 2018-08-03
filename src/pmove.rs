use board;

pub enum Move {
    Mv(board::Pos),
    GiveUp,
    Pass,
}