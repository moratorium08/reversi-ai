use std::cmp::PartialEq;

#[derive(Debug)]
pub struct Player(bool);

impl Player {
    pub fn opposite(self) -> Player {
        let Player(b) = self;
        Player(!b)
    }
    pub fn white() -> Player{
        Player(true)
    }

    pub fn black() -> Player{
        Player(false)
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        let Player(p) = *self;
        let Player(q) = *other;
        p == q
    }
}

