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
    let Some(body) = body(command) else {
        return None;
    };
    let Some(operators) = operators(body) else {
        return None;
    };
    let Some(Operator::Number(acc)) = operators.get(0) else {
        return None;
    };
    operators[1..].chunks(2).try_fold(*acc, |el, chunk| {
        let Operator::Operation(op) = chunk[0] else {
            return None;
        };
        let Some(Operator::Number(val)) = chunk.get(1) else {
            return None;
        };
        Some(op(el, *val))
    })
}

fn body(command: &str) -> Option<&str> {
    const PREFIX: &str = "What is ";
    const SUFFIX: char = '?';
    (command.starts_with(PREFIX) && command.ends_with(SUFFIX))
        .then(|| &command[PREFIX.len()..command.len() - SUFFIX.len_utf8()])
}

fn operators(body: &str) -> Option<Vec<Operator>> {
    body.split_ascii_whitespace()
        .filter(|word| *word != "by")
        .map(Operator::new)
        .collect()
}
