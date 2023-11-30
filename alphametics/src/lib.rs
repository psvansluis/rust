use std::{
    collections::{HashMap, HashSet},
    iter::zip,
};

use itertools::Itertools;

type Solution = HashMap<char, u8>;

pub fn solve(input: &str) -> Option<Solution> {
    let terms: Vec<&str> = input
        .split(&['=', '+'][..])
        .filter(|str| !str.is_empty())
        .map(&str::trim)
        .collect();
    let (lhs, [rhs]) = terms.split_at(terms.len() - 1) else {return None;};
    let characters: HashSet<char> = input.chars().filter(char::is_ascii_uppercase).collect();
    let n_chars = characters.len();
    if n_chars > 10 {
        return None;
    }
    (0_u8..10)
        .permutations(n_chars)
        .map(|permutation| zip(characters.clone(), permutation).collect::<Solution>())
        .find(|solution| {
            no_initial_zeros(solution, &terms)
                && lhs
                    .iter()
                    .map(|term| term_to_number(term, solution))
                    .sum::<u64>()
                    .eq(&term_to_number(rhs, solution))
        })
}

fn term_to_number(term: &str, solution: &Solution) -> u64 {
    term.chars()
        .fold(0, |acc, ch| acc * 10 + *solution.get(&ch).unwrap() as u64)
}

fn no_initial_zeros(solution: &Solution, terms: &[&str]) -> bool {
    solution
        .iter()
        .all(|(ch, num)| num > &0 || !terms.iter().any(|&term| term.starts_with(*ch)))
}
