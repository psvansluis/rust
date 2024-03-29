use as_variant::as_variant;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
enum Operator {
    Number(i32),
    Operation(fn(i32, i32) -> i32),
}

impl Operator {
    fn new(string: &str) -> Option<Operator> {
        if let Ok(number) = string.parse() {
            return Some(Self::Number(number));
        }
        match string {
            "plus" => Some(Self::Operation(Add::add)),
            "minus" => Some(Self::Operation(Sub::sub)),
            "multiplied" => Some(Self::Operation(Mul::mul)),
            "divided" => Some(Self::Operation(Div::div)),
            _ => None,
        }
    }
}

pub fn answer(command: &str) -> Option<i32> {
    let operators: Vec<Operator> = get_body(command).and_then(into_operators)?;
    let init: &i32 = operators
        .get(0)
        .and_then(|el| as_variant!(el, Operator::Number))?;
    operators[1..]
        .chunks(2)
        .try_fold(*init, |el, chunk| match chunk {
            [Operator::Operation(op), Operator::Number(val)] => Some(op(el, *val)),
            _ => None,
        })
}

fn get_body(command: &str) -> Option<&str> {
    const PREFIX: &str = "What is ";
    const SUFFIX: char = '?';
    (command.starts_with(PREFIX) && command.ends_with(SUFFIX))
        .then(|| &command[PREFIX.len()..command.len() - SUFFIX.len_utf8()])
}

fn into_operators(body: &str) -> Option<Vec<Operator>> {
    body.split_ascii_whitespace()
        .filter(|word| *word != "by")
        .map(Operator::new)
        .collect()
}
