extern crate rand;

use std::error::Error;

use crate::troll::reader::{Atom, Expr, expr, Tuple};

use self::rand::Rng;

pub fn eval(roll_def: &str) -> Result<Vec<u16>, &'static str> {
    match expr(roll_def) {
        Ok((_, expr)) => eval_expr(expr),
        Err(error) => Err("Parse error")
    }
}

fn eval_expr(expr: Expr) -> Result<Vec<u16>, &'static str> {
    match expr {
        Expr::Atom(atom) => eval_atom(atom).map(|a| vec![a]),
        Expr::Tuple(tuple) => eval_tuple(tuple)
    }
}

fn eval_atom(atom: Atom) -> Result<u16, &'static str> {
    match atom {
        Atom::ConstVal(c) => Ok(c as u16),
        Atom::Roll(atom) => eval_atom(*atom).map(roll),
        Atom::Sum(tuple) => eval_tuple(tuple).and_then(|v| Ok(v.as_slice().iter().sum()))
    }
}

fn eval_tuple(tuple: Tuple) -> Result<Vec<u16>, &'static str> {
    match tuple {
        Tuple::Dice(count, roll) =>
            eval_atom(*count)
                .and_then(|n| {
                    let mut vec: Vec<u16> = Vec::new();
                    let r = *roll;
                    while vec.len() < n as usize {
                        let res = eval_atom(r.clone());
                        match res {
                            Ok(val) => vec.push(val),
                            Err(e) => return Err(e)
                        }
                    }
                    Ok(vec)
                })
    }
}

fn roll(size: u16) -> u16 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, size) + 1
}

#[test]
fn test_eval_atom() {
    for x in 0..50 {
        match eval("d6") {
            Ok(v) => match v.as_slice() {
                [n] => assert!(n < &7 && n > &0),
                other => assert_eq!(vec![1], other)
            },
            other => assert_eq!(Ok(vec![]), other)
        };
    };
}

#[test]
fn test_eval_tuple() {
    for x in 0..50 {
        match eval("3d6") {
            Ok(v) => match v.as_slice() {
                [a, b, c] => {
                    assert!(a < &7 && a > &0);
                    assert!(b < &7 && b > &0);
                    assert!(c < &7 && c > &0);
                },
                other => assert_eq!(vec![1], other)
            },
            other => assert_eq!(Ok(vec![]), other)
        };
    };
}

#[test]
fn test_eval_sum() {
    for x in 0..50 {
        match eval("sum 3d6") {
            Ok(v) => match v.as_slice() {
                [s] => assert!(s >= &3 && s <= &18),
                other => assert_eq!(vec![1], other)
            },
            other => assert_eq!(Ok(vec![]), other)
        };
    };
}