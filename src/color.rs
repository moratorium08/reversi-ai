#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Color(bool);

impl Color {
    pub fn opposite(self) -> Color {
        let Color(b) = self;
        Color(!b)
    }
    pub fn white() -> Color {
        Color(true)
    }

    pub fn black() -> Color {
        Color(false)
    }

    pub fn is_white(self) -> bool {
        let Color(b) = self;
        b
    }

    pub fn is_black(self) -> bool {
        let Color(b) = self;
        !b
    }
}
