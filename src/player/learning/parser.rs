use board;
use std::vec::Vec;


#[derive(PartialEq, Eq, Debug)]
pub enum MatchResult {
    Black,
    White,
    Draw,
}

pub enum Pos {
    Pass,
    Mv(board::Pos),
}

pub struct History {
    pub result: MatchResult,
    pub poses: Vec<Pos>,
}


named!(black<&str, MatchResult>,
    do_parse!(
        tag!("B") >>
        (MatchResult::Black)
    )
);
named!(white<&str, MatchResult>,
    do_parse!(
        tag!("W") >>
        (MatchResult::White)
    )
);
named!(draw<&str, MatchResult>,
    do_parse!(
        tag!("D") >>
        (MatchResult::Draw)
    )
);

named!(winner<&str, MatchResult>,
        alt!(white|black|draw)
);

fn is_pos_str1(c: char) -> bool {
    match c {
        'A'..='H' => true,
        _ => false
    }
}

fn is_pos_str2(c: char) -> bool {
    match c {
        '1'..='8' => true,
        _ => false
    }
}

named!(pos_str<&str, String>,
    do_parse!(
        p: take_while_m_n!(1, 1, is_pos_str1) >>
        q: take_while_m_n!(1, 1, is_pos_str2) >>
        (p.to_string() + q)
    )
);

named!(mv<&str, Pos>,
    do_parse!(
        p: pos_str >>
        (Pos::Mv(board::Pos::from_str(p).unwrap()))
    )
);


named!(pass<&str, Pos>,
    do_parse!(
        tag!("pass") >>
        (Pos::Pass)
    )
);

named!(pos<&str, Pos>,
    alt!(mv|pass)
);

named!(pos_comma<&str, Pos>,
    do_parse!(
        p: pos >>
        comma >>
        (p)
    )
);

fn add_vec(mut v: Vec<Pos>, p: Pos) -> Vec<Pos> {
    v.push(p);
    v
}

named!(history<&str, Vec<Pos>>,
    do_parse!(
        poses: many0!(pos_comma) >>
        p: pos >>
        (add_vec(poses, p))
    )
);


named!(comma<&str, ()>, do_parse!(tag!(",") >> ()));

named!(pub parse<&str, History>,
    do_parse!(
        w: winner >>
        comma >>
        h: history >>
        tag!("\n") >>
        (History{result: w, poses: h})
    )
);

#[cfg(test)]
mod tests {
    use color;
    use board;
    use super::*;
    #[test]
    fn test_parse_csv_line() {
        let s = "W,pass,H4,B7,B8,A8,pass\n";
        let his = parse(s);
        match his {
            Ok(("", his)) => {
                assert_eq!(his.poses.len(), 6);
                assert_eq!(his.result, MatchResult::White);
            }
            _ => panic!("Failed to parse")
        }

        let long = "W,D3,C5,D6,E3,F4,C6,F5,C3,C4,B5,B4,F3,A5,B6,E6,A6,A7,A3,C7,D7,C8,G4,H3,F7,E7,G6,G3,G5,H5,F8,E8,F6,F2,D2,E2,C2,D1,B3,C1,A4,A2,G2,F1,H1,G1,E1,B2,A1,B1,H2,G8,D8,G7,H8,H7,H6,pass,H4,pass,B7,B8,A8\n";
        let his = parse(long);
        let mut board = board::Board::new();
        let mut player = color::Color::black();
        match his {
            Ok(("", his)) => {
                for pos in his.poses.iter() {
                    match pos {
                        Pos::Pass => (),
                        Pos::Mv(p) => {
                            board = board.flip(p, player);
                        }
                    }
                    player = player.opposite();
                }

                let (black, white) = board.result();
                assert!(black < white);
            }
            _ => panic!("Failed to parse")
        }
    }
}
