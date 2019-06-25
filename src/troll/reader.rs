use nom::character::complete::digit1;
use nom::{Err, Needed};
use nom::error::ErrorKind;

#[derive(Debug, Clone, PartialEq)]
pub enum Atom {
    ConstVal(u8),
    Roll(Box<Atom>),
    Sum(Tuple),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tuple {
    Dice(Box<Atom>, Box<Atom>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Atom(Atom),
    Tuple(Tuple),
}

named!(constval<&str, Atom>,
    map_res!(
        digit1,
        |c| u8::from_str_radix(c, 10).map(Atom::ConstVal)
    )
);

named!(roll<&str, Atom>,
    map!(
        separated_pair!(
            opt!(tag!("1")),
            ws!(char!('d')),
            atom
        ),
        |(_, r)| Atom::Roll(Box::new(r))
    )
);

named!(sum<&str, Atom>,
    complete!(
        map! (
            preceded!(
                ws!(tag!("sum")),
                tuple
            ),
            |t| Atom::Sum(t)
        )
    )
);

named!(atom_raw<&str, Atom>,
    alt!(roll | constval | sum)
);

named!(atom<&str, Atom>,
    alt!(atom_raw | delimited!(tag!("("), atom_raw, tag!(")")))
);

named!(dice<&str, Tuple>,
    map!(
        pair!(
            atom,
            roll
        ),
        |(n, d)| Tuple::Dice(Box::new(n), Box::new(d))
    )
);

named!(tuple<&str, Tuple>,
    complete!(dice)
);

named!(pub expr<&str, Expr>,
    alt!(
        map!(tuple, Expr::Tuple) |
        map!(atom, Expr::Atom)
    )
);

#[test]
fn test_parse_atom() {
    assert_eq!(constval("123"), Ok(("", Atom::ConstVal(123))));
    assert_eq!(constval("123abc"), Ok(("abc", Atom::ConstVal(123))));
    assert_eq!(constval("1234"), Err(Err::Error(("1234", ErrorKind::MapRes))));
    assert_eq!(roll("1d6"), Ok(("", Atom::Roll(Box::new(Atom::ConstVal(6))))));
    assert_eq!(roll("d6"), Ok(("", Atom::Roll(Box::new(Atom::ConstVal(6))))));
    assert_eq!(roll("dd6"), Ok(("", Atom::Roll(Box::new(Atom::Roll(Box::new(Atom::ConstVal(6))))))));
    assert_eq!(sum(" sum  3d6"), Ok(("", Atom::Sum(Tuple::Dice(Box::new(Atom::ConstVal(3)), Box::new(Atom::Roll(Box::new(Atom::ConstVal(6)))))))));
}

#[test]
fn test_parse_dice() {
    assert_eq!(dice("3d6"), Ok(("", Tuple::Dice(Box::new(Atom::ConstVal(3)), Box::new(Atom::Roll(Box::new(Atom::ConstVal(6))))))));
    assert_eq!(dice("3 d 6"), Ok(("", Tuple::Dice(Box::new(Atom::ConstVal(3)), Box::new(Atom::Roll(Box::new(Atom::ConstVal(6))))))));
    assert_eq!(dice("(1d6) d d6"), Ok(("", Tuple::Dice(
        Box::new(Atom::Roll(Box::new(Atom::ConstVal(6)))),
        Box::new(Atom::Roll(Box::new(Atom::Roll(Box::new(Atom::ConstVal(6)))))),
    ))));
    assert_eq!(dice("3d"), Err(Err::Incomplete(Needed::Size(1))));
}

#[test]
fn test_parse_expr() {
    assert_eq!(expr("(1d6) d d6"), Ok(("", Expr::Tuple(Tuple::Dice(
        Box::new(Atom::Roll(Box::new(Atom::ConstVal(6)))),
        Box::new(Atom::Roll(Box::new(Atom::Roll(Box::new(Atom::ConstVal(6)))))),
    )))));

    assert_eq!(expr("d6"), Ok(("", Expr::Atom(Atom::Roll(Box::new(Atom::ConstVal(6)))))));
}