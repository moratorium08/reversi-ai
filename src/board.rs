use std::fmt;
use std::vec;
use std::iter::Iterator;


use color::Color;
use util::clz;


#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Pos {
    x: u8,
    y: u8,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Flippable(pub u64);

impl Flippable {
    pub fn poses(&self) -> vec::Vec<Pos> {
        let Flippable(x) = *self;
        let mut v: vec::Vec<Pos> = vec![];

        for i in 0u8..64u8 {
            if x & (1 << i) != 0 {
                v.push(Pos::from_index(i));
            }
        }
        v
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    white: u64,
    black: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Rotate {
    Rotate0cw,
    Rotate90cw,
    Rotate180cw,
    Rotate270cw,
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
            } else if c1 > 56 || c1 < 49 {
                Err("位置の範囲外".to_string())
            } else {
                Ok(Pos { y: c0 - 65, x: c1 - 49 })
            }
        }
    }

    pub fn to_string(self) -> String {
        let c0 = (self.y + 65) as char;
        let c1 = (self.x + 49) as char;
        let mut s = String::new();
        s.push(c0);
        s.push(c1);
        s
    }

    pub fn from_index(index: u8) -> Pos {
        Pos { x: index / 8, y: index % 8 }
    }

    pub fn from_point(x: u8, y: u8) -> Pos {
        Pos { x, y }
    }
}

pub trait BitIndexable {
    fn to_index(&self) -> u8;
}

impl BitIndexable for Pos {
    fn to_index(&self) -> u8 {
        self.x * 8 + self.y
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Hash([u8; 16]);

impl Hash {
    pub fn from_values(white: u64, black: u64) -> Hash {
        let mut white = white;
        let mut black = black;
        let mut ret = [0u8; 16];
        for i in 0..8 {
            ret[7 - i] = (white & 0xffu64) as u8;
            white >>= 8;
        }
        for i in 0..8 {
            ret[15 - i] = (black & 0xffu64) as u8;
            black >>= 8;
        }
        Hash(ret)
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Hash(data) = *self;
        for (i, x) in data.iter().enumerate() {
            match if i == 7 { write!(f, "{:02x}, ", x) } else { write!(f, "{:02x}", x) } {
                Ok(_) => (),
                x => return x
            }
        }
        Ok(())
    }
}

macro_rules! gen_pattern_func{
    ($name:ident, $size:expr, $vec:expr) => {
        pub fn $name(&self, rotate: Rotate) -> usize {
            let board = self.rotate(rotate);

            const PS: [u8; $size] = $vec;
            let mut ret = 0usize;
            for p in PS.iter() {
                ret *= 3;
                ret += board.encode_pos(*p) as usize;
            }
            ret
        }
    }
}

impl Board {
    pub fn new() -> Board {
        Board {
            white: (1 << 27) + (1 << 36),
            black: (1 << 28) + (1 << 35),
        }
    }

    pub fn hash(&self) -> Hash {
        let mut tmp_white = self.white;
        let mut tmp_black = self.black;
        let mut ret = [0u8; 16];
        for i in 0..8 {
            ret[7 - i] = (tmp_white & 0xffu64) as u8;
            tmp_white >>= 8;
        }
        for i in 0..8 {
            ret[15 - i] = (tmp_black & 0xffu64) as u8;
            tmp_black >>= 8;
        }
        Hash(ret)
    }

    pub fn color<T: BitIndexable>(&self, pos: T) -> Option<Color> {
        let i = pos.to_index();
        if (1 << i) & self.white != 0 {
            Some(Color::white())
        } else if (1 << i) & self.black != 0 {
            Some(Color::black())
        } else {
            None
        }
    }

    pub fn from_hash(hash: Hash) -> Board {
        let Hash(hash) = hash;
        let mut white = 0u64;
        for i in 0..8 {
            white <<= 8;
            white += hash[i] as u64;
        }
        let mut black = 0u64;
        for i in 0..8 {
            black <<= 8;
            black += hash[8 + i] as u64;
        }
        Board { white, black }
    }

    pub fn to_string(&self) -> String {
        let mut ret = String::new();
        for i in 0..64 {
            if i > 0 && i % 8 == 0 {
                ret.push_str("\n");
            }
            if ((1 << i) & self.black) != 0 {
                ret.push_str("x");
            } else if ((1 << i) & self.white) != 0 {
                ret.push_str("o");
            } else {
                ret.push_str(" ");
            }
        }
        ret
    }

    fn print_flippable_board(&self, fl: Flippable) {
        let Flippable(flippable) = fl;
        println!("  A B C D E F G H");
        println!("  ---------------");
        for i in 0..64 {
            if i % 8 == 0 {
                if i > 0 {
                    print!("\n");
                }
                print!("{} ", i / 8 + 1);
            }

            if ((1 << i) & self.black) != 0 {
                print!("x ");
            } else if ((1 << i) & self.white) != 0 {
                print!("o ");
            } else if ((1 << i) & flippable) != 0 {
                print!(". ");
            } else {
                print!("  ");
            }
        }
        print!("\n");
    }

    pub fn print_player_board(&self, p: Color) {
        self.print_flippable_board(self.flippable(p));
    }

    pub fn print(&self) {
        self.print_flippable_board(Flippable(0));
    }

    #[inline(always)]
    pub fn result(&self) -> (u8, u8) {
        (self.black.count_ones() as u8, self.white.count_ones() as u8)
    }

    pub fn flip(&self, p: &BitIndexable, player: Color) -> Board {
        let (pl, op) =
            if player.is_white() {
                (self.white, self.black)
            } else {
                (self.black, self.white)
            };
        let pos = p.to_index();

        let flipped = self.gen_flip(pos, pl, op);

        let next_pl = pl | (1u64 << pos) | flipped;
        let next_op = op & (!flipped);

        if player.is_white() {
            Board { white: next_pl, black: next_op }
        } else {
            Board { white: next_op, black: next_pl }
        }
    }

    // ref: http://primenumber.hatenadiary.jp/entry/2016/12/26/063226
    fn gen_flip(&self, pos: u8, pl: u64, op: u64) -> u64 {
        let x = op;
        let yzw = op & 0x7e7e7e7e7e7e7e7eu64;

        let maskx = 0x0080808080808080u64 >> (63 - pos);
        let masky = 0x7f00000000000000u64 >> (63 - pos);
        let maskz = 0x0102040810204000u64 >> (63 - pos);
        let maskw = 0x0040201008040201u64 >> (63 - pos);

        let outflankx = (0x8000000000000000u64 >> clz(!x & maskx)) & pl;
        let outflanky = (0x8000000000000000u64 >> clz(!yzw & masky)) & pl;
        let outflankz = (0x8000000000000000u64 >> clz(!yzw & maskz)) & pl;
        let outflankw = (0x8000000000000000u64 >> clz(!yzw & maskw)) & pl;

        let flippedx = (((-(outflankx as i64)) * 2) as u64) & maskx;
        let flippedy = (((-(outflanky as i64)) * 2) as u64) & masky;
        let flippedz = (((-(outflankz as i64)) * 2) as u64) & maskz;
        let flippedw = (((-(outflankw as i64)) * 2) as u64) & maskw;

        let mask2x = 0x0101010101010100u64 << pos;
        let mask2y = 0x00000000000000feu64 << pos;
        let mask2z = 0x0002040810204080u64 << pos;
        let mask2w = 0x8040201008040200u64 << pos;

        // releaseビルドの時は使うようにする
        let outflank2x = mask2x & ((x | !mask2x) + 1) & pl;
        let outflank2y = mask2y & ((yzw | !mask2y) + 1) & pl;
        let outflank2z = mask2z & ((yzw | !mask2z) + 1) & pl;
        let outflank2w = mask2w & ((yzw | !mask2w) + 1) & pl;
        /*let outflank2x = mask2x & (Wrapping(x   | !mask2x) + Wrapping(1)).0 & pl;
        let outflank2y = mask2y & (Wrapping(yzw | !mask2y) + Wrapping(1)).0 & pl;
        let outflank2z = mask2z & (Wrapping(yzw | !mask2z) + Wrapping(1)).0 & pl;
        let outflank2w = mask2w & (Wrapping(yzw | !mask2w) + Wrapping(1)).0 & pl;*/


        let outflank2x = ((outflank2x as i64) - ((outflank2x != 0) as i64)) as u64;
        let outflank2y = ((outflank2y as i64) - ((outflank2y != 0) as i64)) as u64;
        let outflank2z = ((outflank2z as i64) - ((outflank2z != 0) as i64)) as u64;
        let outflank2w = ((outflank2w as i64) - ((outflank2w != 0) as i64)) as u64;


        let flipped2x = flippedx | (outflank2x & mask2x);
        let flipped2y = flippedy | (outflank2y & mask2y);
        let flipped2z = flippedz | (outflank2z & mask2z);
        let flipped2w = flippedw | (outflank2w & mask2w);

        flipped2x | flipped2y | flipped2z | flipped2w
    }

    pub fn flippable(&self, player: Color) -> Flippable {
        let (pl, op) = if player.is_white() { (self.white, self.black) } else { (self.black, self.white) };

        // TODO: Software Pipelining

        let x = 0x7e7e7e7e7e7e7e7eu64 & op;
        let y = 0x00FFFFFFFFFFFF00u64 & op;
        let z = 0x007e7e7e7e7e7e00u64 & op;
        let blank = !(pl | op);

        let tmp1 = x & (pl << 1);
        let tmp2 = x & (pl >> 1);
        let tmp3 = y & (pl << 8);
        let tmp4 = y & (pl >> 8);

        let tmp1 = tmp1 | (x & (tmp1 << 1));
        let tmp2 = tmp2 | (x & (tmp2 >> 1));
        let tmp3 = tmp3 | (y & (tmp3 << 8));
        let tmp4 = tmp4 | (y & (tmp4 >> 8));

        let tmp1 = tmp1 | (x & (tmp1 << 1));
        let tmp2 = tmp2 | (x & (tmp2 >> 1));
        let tmp3 = tmp3 | (y & (tmp3 << 8));
        let tmp4 = tmp4 | (y & (tmp4 >> 8));

        let tmp1 = tmp1 | (x & (tmp1 << 1));
        let tmp2 = tmp2 | (x & (tmp2 >> 1));
        let tmp3 = tmp3 | (y & (tmp3 << 8));
        let tmp4 = tmp4 | (y & (tmp4 >> 8));

        let tmp1 = tmp1 | (x & (tmp1 << 1));
        let tmp2 = tmp2 | (x & (tmp2 >> 1));
        let tmp3 = tmp3 | (y & (tmp3 << 8));
        let tmp4 = tmp4 | (y & (tmp4 >> 8));

        let tmp1 = tmp1 | (x & (tmp1 << 1));
        let tmp2 = tmp2 | (x & (tmp2 >> 1));
        let tmp3 = tmp3 | (y & (tmp3 << 8));
        let tmp4 = tmp4 | (y & (tmp4 >> 8));

        let flippable1 = blank & (tmp1 << 1);
        let flippable2 = blank & (tmp2 >> 1);
        let flippable3 = blank & (tmp3 << 8);
        let flippable4 = blank & (tmp4 >> 8);

        let tmp5 = z & (pl << 7);
        let tmp6 = z & (pl >> 7);
        let tmp7 = z & (pl << 9);
        let tmp8 = z & (pl >> 9);

        let tmp5 = tmp5 | (z & (tmp5 << 7));
        let tmp6 = tmp6 | (z & (tmp6 >> 7));
        let tmp7 = tmp7 | (z & (tmp7 << 9));
        let tmp8 = tmp8 | (z & (tmp8 >> 9));

        let tmp5 = tmp5 | (z & (tmp5 << 7));
        let tmp6 = tmp6 | (z & (tmp6 >> 7));
        let tmp7 = tmp7 | (z & (tmp7 << 9));
        let tmp8 = tmp8 | (z & (tmp8 >> 9));

        let tmp5 = tmp5 | (z & (tmp5 << 7));
        let tmp6 = tmp6 | (z & (tmp6 >> 7));
        let tmp7 = tmp7 | (z & (tmp7 << 9));
        let tmp8 = tmp8 | (z & (tmp8 >> 9));

        let tmp5 = tmp5 | (z & (tmp5 << 7));
        let tmp6 = tmp6 | (z & (tmp6 >> 7));
        let tmp7 = tmp7 | (z & (tmp7 << 9));
        let tmp8 = tmp8 | (z & (tmp8 >> 9));


        let tmp5 = tmp5 | (z & (tmp5 << 7));
        let tmp6 = tmp6 | (z & (tmp6 >> 7));
        let tmp7 = tmp7 | (z & (tmp7 << 9));
        let tmp8 = tmp8 | (z & (tmp8 >> 9));

        let flippable5 = blank & (tmp5 << 7);
        let flippable6 = blank & (tmp6 >> 7);
        let flippable7 = blank & (tmp7 << 9);
        let flippable8 = blank & (tmp8 >> 9);

        Flippable(flippable1 | flippable2 | flippable3 | flippable4 | flippable5 | flippable6 | flippable7 | flippable8)
    }

    fn is_valid(&self, pos: &BitIndexable, player: Color) -> bool {
        let (pl, op) =
            if player.is_black() {
                (self.black, self.white)
            } else {
                (self.white, self.black)
            };
        let pos = pos.to_index();

        self.gen_flip(pos, pl, op) != 0
    }

    // https://ameblo.jp/n-amane-n/entry-12305741801.html
    // ちょっと怠惰すぎるが時間ないので
    pub fn rotate(&self, rotate: Rotate) -> Board {
        let mut b = *self;
        match rotate {
            Rotate::Rotate0cw => { return *self; }
            Rotate::Rotate90cw => {}
            Rotate::Rotate180cw => { b = self.rotate(Rotate::Rotate90cw); }
            Rotate::Rotate270cw => { b = self.rotate(Rotate::Rotate180cw); }
        }

        let tmp = b.white;
        let mut white = 0x00000000F0F0F0F0u64 & (tmp << 4);
        white |= 0xF0F0F0F00F0F0F0Fu64 & (tmp << 32);
        white |= 0xF0F0F0F00F0F0F0Fu64 & (tmp >> 32);
        white |= 0x0F0F0F0F00000000u64 & (tmp >> 4);

        let tmp = white;
        white = 0x0000CCCC0000CCCC & (tmp << 2);
        white |= 0xCCCC0000CCCC0000 & (tmp << 16);
        white |= 0x0000333300003333 & (tmp >> 16);
        white |= 0x3333000033330000 & (tmp >> 2);

        let tmp = white;
        white = 0x00AA00AA00AA00AA & (tmp << 1);
        white |= 0xAA00AA00AA00AA00 & (tmp << 8);
        white |= 0x0055005500550055 & (tmp >> 8);
        white |= 0x5500550055005500 & (tmp >> 1);

        let tmp = b.black;
        let mut black = 0x00000000F0F0F0F0u64 & (tmp << 4);
        black |= 0xF0F0F0F00F0F0F0Fu64 & (tmp << 32);
        black |= 0xF0F0F0F00F0F0F0Fu64 & (tmp >> 32);
        black |= 0x0F0F0F0F00000000u64 & (tmp >> 4);

        let tmp = black;
        black = 0x0000CCCC0000CCCC & (tmp << 2);
        black |= 0xCCCC0000CCCC0000 & (tmp << 16);
        black |= 0x0000333300003333 & (tmp >> 16);
        black |= 0x3333000033330000 & (tmp >> 2);

        let tmp = black;
        black = 0x00AA00AA00AA00AA & (tmp << 1);
        black |= 0xAA00AA00AA00AA00 & (tmp << 8);
        black |= 0x0055005500550055 & (tmp >> 8);
        black |= 0x5500550055005500 & (tmp >> 1);

        Board{white, black}
    }

    fn encode_pos(&self, bit: u8) -> u8 {
        ((self.black >> bit) & 1 + ((self.white >> bit) & 1) * 2) as u8
    }

    // TODO: マクロ化
    gen_pattern_func!(diag4, 4, [3, 10, 17, 24]);
    gen_pattern_func!(diag5, 5, [4, 11, 18, 25, 32]);
    gen_pattern_func!(diag6, 6, [5, 12, 19, 26, 33, 40]);
    gen_pattern_func!(diag7, 7, [6, 13, 20, 27, 34, 41, 48]);

    pub fn diag7(&self, rotate: Rotate) -> usize {
        let board = self.rotate(rotate);

        const PS: [u8; 7] = [6, 13, 20, 27, 34, 41, 48];
        let mut ret = 0usize;
        for p in PS.iter() {
            ret *= 3;
            ret += board.encode_pos(*p) as usize;
        }
        ret
    }

    pub fn diag8(&self, rotate: Rotate) -> usize {
        let board = self.rotate(rotate);

        const PS: [u8; 8] = [7, 14, 21, 28, 35, 42, 49, 56];
        let mut ret = 0usize;
        for p in PS.iter() {
            ret *= 3;
            ret += board.encode_pos(*p) as usize;
        }
        ret
    }

    pub fn hor_vert2(&self, rotate: Rotate) -> usize {
        let board = self.rotate(rotate);

        const PS: [u8; 8] = [8, 9, 10, 11, 12, 13, 14, 15];
        let mut ret = 0usize;
        for p in PS.iter() {
            ret *= 3;
            ret += board.encode_pos(*p) as usize;
        }
        ret
    }

    pub fn hor_vert3(&self, rotate: Rotate) -> usize {
        let board = self.rotate(rotate);

        const PS: [u8; 8] = [16, 17, 18, 19, 20, 21, 22, 23];
        let mut ret = 0usize;
        for p in PS.iter() {
            ret *= 3;
            ret += board.encode_pos(*p) as usize;
        }
        ret
    }

    pub fn hor_vert4(&self, rotate: Rotate) -> usize {
        let board = self.rotate(rotate);

        const PS: [u8; 8] = [24, 25, 26, 27, 28, 29, 30, 31];
        let mut ret = 0usize;
        for p in PS.iter() {
            ret *= 3;
            ret += board.encode_pos(*p) as usize;
        }
        ret
    }

    pub fn edge2x(&self, rotate: Rotate) -> usize {
        let board = self.rotate(rotate);

        const PS: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 9, 14];
        let mut ret = 0usize;
        for p in PS.iter() {
            ret *= 3;
            ret += board.encode_pos(*p) as usize;
        }
        ret
    }

    pub fn corner2x5(&self, rotate: Rotate) -> usize {
        let board = self.rotate(rotate);

        const PS: [u8; 10] = [0, 1, 2, 3, 4, 8, 9, 10, 11, 12];
        let mut ret = 0usize;
        for p in PS.iter() {
            ret *= 3;
            ret += board.encode_pos(*p) as usize;
        }
        ret
    }

    pub fn corner3x3(&self, rotate: Rotate) -> usize {
        let board = self.rotate(rotate);

        const PS: [u8; 9] = [0, 1, 2, 8, 9, 10, 16, 17, 18];
        let mut ret = 0usize;
        for p in PS.iter() {
            ret *= 3;
            ret += board.encode_pos(*p) as usize;
        }
        ret
    }
}
