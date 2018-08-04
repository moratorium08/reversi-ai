use board;

trait Evaluator {
    fn eval(board: board::Board) -> i64;
}
