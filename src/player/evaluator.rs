use board;

pub trait Evaluator {
    fn evaluate(&self, board: board::Board) -> i64;
}
