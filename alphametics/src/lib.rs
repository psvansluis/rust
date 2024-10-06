use std::{
    collections::{HashMap, HashSet},
    iter::zip,
};

use itertools::Itertools;

type Solution = HashMap<char, u8>;

pub fn solve(input: &str) -> Option<Solution> {
    let characters: HashSet<char> = input.chars().filter(char::is_ascii_uppercase).collect();
    let n_chars = characters.len();
    if n_chars > 10 {
        return None;
    }
    let terms: Vec<&str> = input
        .split(&['=', '+'][..])
        .filter(|str| !str.is_empty())
        .map(&str::trim)
        .collect();
    let (lhs, [rhs]) = terms.split_at(terms.len() - 1) else {return None;};
    let no_initial_zeros = |solution: &Solution| -> bool {
        solution
            .iter()
            .all(|(ch, num)| num > &0 || !terms.iter().any(|&term| term.starts_with(*ch)))
    };
    let lhs_eq_rhs = |solution: &Solution| -> bool {
        let to_num = |term: &&str| {
            term.chars()
                .fold(0, |acc, ch| acc * 10 + *solution.get(&ch).unwrap() as u64)
        };
        lhs.iter().map(to_num).sum::<u64>().eq(&to_num(rhs))
    };
    (0_u8..10)
        .permutations(n_chars)
        .map(|permutation| zip(characters.clone(), permutation).collect::<Solution>())
        .find(|solution: &Solution| no_initial_zeros(solution) && lhs_eq_rhs(solution))
}
