pub trait Client {
    fn game_loop(&self);
    fn name(&self) -> String;
}
