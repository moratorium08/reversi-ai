use board;

pub enum Move {
    Mv(Pos),
    Giveup,
    Pass,
}