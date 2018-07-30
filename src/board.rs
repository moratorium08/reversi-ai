pub struct Pos {
    x: u8,
    y: u8,
}

pub struct Board {
    white: u64,
    black: u64,
}

impl Pos {
    pub fn from_str(s: String) -> Result<Pos, String> {
        let cs: Vec<char> = s.chars().collect();
        if cs.len() != 2 {
            Err("位置の形式エラー".to_string())
        } else {
            let c0 = cs[0] as u8;
            let c1 = cs[1] as u8;
            if c0 > 72 || c0 < 65 {
                Err("位置の範囲外".to_string())
            }
            else if c1 > 56 || c1 < 49 {
                Err("位置の範囲外".to_string())
            } else {
                Ok(Pos{x: c0 - 65, y: c1 - 49})
            }
        }
    }

    pub fn to_string(self) -> String {
        let c0 = (self.x + 65) as char;
        let c1 = (self.y + 49) as char;
        let mut s = String::new();
        s.push(c0);
        s.push(c1);
        s
    }
}

trait BitIndexable {
    fn to_index(&self) -> u8;
}

impl BitIndexable for Pos {
    fn to_index(&self) -> u8 {
        self.x * 8 + self.y
    }
}

impl Board {
    pub fn new() -> Board {
        Board{
            white: (1 << 27) + (1 << 36),
            black: (1 << 28) + (1 << 35),
        }
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        for i in 0..64 {
            if i > 0 && i % 8 == 0 {
                ret.push_str("\n");
            }
            if ((1 << i) & self.black) != 0 {
                ret.push_str("o");
            } else if ((1 << i) & self.white) != 0{
                ret.push_str("x");
            } else {
                ret.push_str(" ");
            }
        }
        ret
    }

    // ref: http://primenumber.hatenadiary.jp/entry/2016/12/26/063226
    /* fn flip(&self, p: &BitIndexable) -> Board {
        let pos = p.to_index();
        let x = self.white;
        let yzw = self.white & 0x7e7e7e7e7e7e7e7e;

        let maskx = 0x0080808080808080u64 >> (63 - pos);
        let masky = 0x7f00000000000000u64 >> (63 - pos);
        let maskz = 0x0102040810204000u64 >> (63 - pos);
        let maskz = 0x0040201008040201u64>> (63 - pos);
        let outflank = (0x8000000000000000u64 >> clz(~OM & mask)) & P;

        *self
    }*/

    /*fn is_valid(&self, p: &BitIndexable) -> bool {
        let index = p.to_index();
        if ((1 << index) & self.white) != 0 ||
            ((1 << index) & self.black) != 0 {
            false
        } else {
        }
    }

    pub fn put_pos(&self, p: &BitIndexable) -> Board {

    }*/
}
