use std::{collections::HashMap, str::Chars};

#[derive(Default)]
pub struct Polynome {
    coefficients: HashMap<u8, i128>,
}

impl From<String> for Polynome {
    fn from(input: String) -> Self {
        let mut ret = Polynome::default();

        let pieces = input.split('=').collect::<Vec<_>>();
        match pieces.len() {
            x if x > 2 => panic!("Too much equal signs."),
            x if x < 2 => panic!("Not enough equal signs"),
            _ => {}
        }

        if let [left, right] = pieces[..] {
            let left = left.split_whitespace().collect::<Vec<_>>();
            let right = right.split_whitespace().collect::<Vec<_>>();

            println!("{:?}", left);
            println!("{:?}", right);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn empty_string() {
        let _: Polynome = String::from("").into();
    }

    #[test]
    #[should_panic]
    fn two_equal_signs() {
        let _: Polynome = String::from("1 = 2 = 3").into();
    }

    #[test]
    fn simple() {
        let _: Polynome = String::from("1 * X^0 + 4 * X^1 = 3 * X^0").into();
    }

    #[test]
    fn bonus() {
        let _: Polynome = String::from("4x^2 - 2x + 2 = 27x").into();
    }
}
