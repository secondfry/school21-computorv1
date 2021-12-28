use core::panic;
use std::{collections::HashMap, env};

use crate::polynome::Polynome;

mod polynome;

fn receive_equation_string() -> String {
    let mut args = env::args().skip(1);

    match args.next() {
        Some(x) => x,
        None => panic!("Please provide an equation as argument to program."),
    }
}

fn main() {
    let poly: Polynome = receive_equation_string().into();
    println!("Hello, world!");
}
