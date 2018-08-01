use std::cmp::PartialEq;

#[derive(Clone, Copy, Debug, Eq, Hash)]
pub struct Player(bool);

impl Player {
    pub fn opposite(self) -> Player {
        let Player(b) = self;
        Player(!b)
    }
    pub fn white() -> Player {
        Player(true)
    }

    pub fn black() -> Player {
        Player(false)
    }

    pub fn is_white(self) -> bool {
        let Player(b) = self;
        b
    }

    pub fn is_black(self) -> bool {
        let Player(b) = self;
        !b
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        let Player(p) = *self;
        let Player(q) = *other;
        p == q
    }
}
