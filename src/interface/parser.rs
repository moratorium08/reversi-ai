use std::str::FromStr;
use std::vec::Vec;

use nom::{digit, space};

use interface::client::{Color, Command, MatchResult, Move};

named!(sp<&str, ()>,
    do_parse!(
        is_a!(" \t") >> ()
        )
);

named!(string<&str, &str>,
    do_parse!(
        ret: is_not!(" \n\t") >>
        (ret)
    )
);

named!(open_command<&str, Command>,
       do_parse!(
           ws!(tag!("OPEN")) >>
           name: string >>
           tag!("\n") >>
           (Command::Open(String::from(name)))
       )
);

#[test]
fn test_open_command() {
    match open_command("OPEN hoge\n") {
        Ok(cmd) => match cmd {
            ("", Command::Open(s)) => {
                assert_eq!(s, "hoge".to_string());
            }
            _ => {
                panic!("should be open command");
            }
        },
        _ => {
            panic!("should be parsed");
        }
    }
}

named!(win<&str, MatchResult>,
    do_parse!(
        ws!(tag!("WIN")) >>
        (MatchResult::Win)
));

named!(lose<&str, MatchResult>,
    do_parse!(
        ws!(tag!("LOSE")) >>
        (MatchResult::Lose)
));

named!(tie<&str, MatchResult>,
    do_parse!(
        ws!(tag!("TIE")) >>
        (MatchResult::Tie)
));

named!(win_lose_tie<&str, MatchResult>,
    alt!(win|lose|tie)
);

named!(uint8<&str, u8>,
    map_res!(
        ws!(digit),
        FromStr::from_str
    )
);

named!(uint64<&str, u64>,
    map_res!(
        digit,
        FromStr::from_str
    )
);

named!(end_command<&str, Command>,
       do_parse!(
           ws!(tag!("END")) >>
           wl: win_lose_tie >>
           pl: uint8 >>
           op: uint8 >>
           res: string >>
           many0!(sp) >>
           tag!("\n") >>
           (Command::End(wl, pl, op, res.to_string()))
       )
);


#[test]
fn test_end_command() {
    let table = [
        ("END WIN 10 20 hoge\n", &MatchResult::Win, 10u8, 20u8, &"hoge".to_string()),
        ("END LOSE 30 20 piyo\n", &MatchResult::Lose, 30u8, 20u8, &"piyo".to_string()),
        ("END TIE 30 20 piyo\n", &MatchResult::Tie, 30u8, 20u8, &"piyo".to_string()),
    ];

    for &(s, w, p, o, r) in table.iter() {
        match end_command(s) {
            Ok(cmd) => match cmd {
                ("", Command::End(wl, pl, op, res)) => {
                    assert_eq!(*w, wl);
                    assert_eq!(p, pl);
                    assert_eq!(o, op);
                    assert_eq!(*r, res);
                }
                _ => {
                    panic!("should be end command");
                }
            },
            _ => {
                panic!("should be parsed");
            }
        }
    }
}


named!(score<&str, (String, (u64, u64, u64))>,
    do_parse!(
        many0!(sp) >>
        st: string >>
        many1!(sp) >>
        x: uint64 >>
        many1!(sp) >>
        y: uint64 >>
        many1!(sp) >>
        z: uint64 >>
        ((st.to_string(), (x, y, z)))
    )
);

named!(scores<&str, Vec<(String, (u64, u64, u64))>>,
    do_parse!(
        ret: many0!(score) >>
        (ret)
    )
);

named!(bye_command<&str, Command>,
       do_parse!(
           tag!("BYE") >>
           many1!(sp) >>
           sc: scores >>
           tag!("\n") >>
           (Command::Bye(sc))
       )
);

#[test]
fn test_bye_command() {
    let table = [
        ("BYE hoge \t 1 2 3\n", &(vec![("hoge", (1u64, 2u64, 3u64))])),
        ("BYE hoge \t 1 2 3 fuga 3 4 5\n", &(vec![
            ("hoge", (1u64, 2u64, 3u64)),
            ("fuga", (3u64, 4u64, 5u64))
        ]))
    ];

    for &(s, v) in table.iter() {
        match bye_command(s) {
            Ok(cmd) => match cmd {
                ("", Command::Bye(w)) => {
                    assert_eq!(w.len(), v.len());
                }
                (x, Command::Bye(v)) => { panic!("hoge={}", x); }
                _ => {
                    panic!("should be end command");
                }
            },
            _ => {
                panic!("should be parsed");
            }
        }
    }
}


named!(ack_command<&str, Command>,
       do_parse!(
        tag!("ACK") >>
        many1!(sp) >>
        time: uint64 >>
        many0!(sp) >>
        tag!("\n") >>
        (Command::Ack(time))
    )
);

#[test]
fn test_ack_command() {
    match ack_command("ACK 1025\n") {
        Ok(cmd) => match cmd {
            ("", Command::Ack(s)) => {
                assert_eq!(s, 1025u64);
            }
            _ => {
                panic!("should be open command");
            }
        },
        _ => {
            panic!("should be parsed");
        }
    }
}
named!(color<&str, Color>,
    alt!(
        do_parse!(tag!("WHITE") >> (Color::White))|
        do_parse!(tag!("BLACK") >> (Color::Black))
    )
);

named!(start_command<&str, Command>,
       do_parse!(
        tag!("START") >>
        many1!(sp) >>
        col: color >>
        many1!(sp) >>
        name: string >>
        many1!(sp) >>
        time: uint64 >>
        tag!("\n") >>
        (Command::Start(col, name.to_string(), time))
    )
);

#[test]
fn test_start_command() {
    match start_command("START WHITE player 1025\n") {
        Ok(cmd) => match cmd {
            ("", Command::Start(wl, pl, ti)) => {
                assert_eq!(wl, Color::White);
                assert_eq!(pl, "player".to_string());
                assert_eq!(ti, 1025u64);
            }
            _ => {
                panic!("should be open command");
            }
        },
        _ => {
            panic!("should be parsed");
        }
    }
}

named!(pass<&str, Move>,
    do_parse!(
        tag!("PASS") >>
        (Move::Pass)
    )
);

named!(giveup<&str, Move>,
    do_parse!(
        tag!("GIVEUP") >>
        (Move::GiveUp)
    )
);

named!(mv<&str, Move>,
    do_parse!(
        pos: take!(2) >>
        (Move::Mv(pos.to_string()))
    )
);

named!(move_command<&str, Command>,
       do_parse!(
        tag!("MOVE") >>
        many1!(sp) >>
        mv: alt!(pass|giveup|mv) >>
        many0!(sp) >>
        tag!("\n") >>
        (Command::Move(mv))
    )
);

#[test]
fn test_move_command() {
    let table = [
        ("MOVE PASS\n", &Move::Pass),
        ("MOVE GIVEUP\n", &Move::GiveUp),
        ("MOVE A1\n", &Move::Mv("A1".to_string()))
    ];

    for &(s, m) in table.iter() {
        match move_command(s) {
            Ok(cmd) => match cmd {
                ("", Command::Move(n)) => {
                    assert_eq!(*m, n);
                }
                _ => {
                    panic!("should be open command");
                }
            },
            _ => {
                panic!("should be parsed");
            }
        }
    }
}

named!(empty_command<&str, Command>,
       do_parse!(
        many0!(sp) >>
        tag!("\n") >>
        (Command::Empty)
    )
);

named!(command_parser<&str, Command>, alt!(open_command|end_command|move_command|ack_command|start_command|bye_command|empty_command));

#[test]
fn test_command_parser() {
    match command_parser("MOVE B3\n") {
        Ok(("", Command::Move(_))) => (),
        _ => panic!("Failed to parse move command"),
    }
    match command_parser("OPEN hello\n") {
        Ok(("", Command::Open(_))) => (),
        _ => panic!("Failed to parse open command"),
    }
    match command_parser("START BLACK pl 1\n") {
        Ok(("", Command::Start(_, _, _))) => (),
        _ => panic!("Failed to parse start command"),
    }
    match command_parser("BYE neko 1 2 3\n") {
        Ok(("", Command::Bye(_))) => (),
        _ => panic!("Failed to parse bye command"),
    }
    match command_parser("ACK 3\n") {
        Ok(("", Command::Ack(_))) => (),
        _ => panic!("Failed to parse ack command"),
    }
    match command_parser("END WIN 1 2 piyo \n") {
        Ok(("", Command::End(_, _, _, _))) => (),
        _ => panic!("Failed to parse end command"),
    }
}


pub fn parse(cmd: String) -> Result<Command, String> {
    match command_parser(&cmd) {
        Ok(("", cmd)) => Ok(cmd),
        _ => Err("Failed to parse")
    }
}
