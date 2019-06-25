#[macro_use]
extern crate nom;

use crate::troll::eval::eval;

mod troll;

fn main() {
    match eval("sum 3d6") {
        Ok(res) => println!("sum 3d6 = {:?}", res),
        Err(e) => println!("Error: {}", e)
    }

    match eval("10d10") {
        Ok(res) => println!("10d10 = {:?}", res),
        Err(e) => println!("Error: {}", e)
    }

    match eval("(1d6) d d6") {
        Ok(res) => println!("(1d6) d d6 = {:?}", res),
        Err(e) => println!("Error: {}", e)
    }
}
