use std::cmp::{max, min};

#[macro_use(c)]
extern crate cute;

pub fn annotate(maes_ffrwydron: &[&str]) -> Vec<String> {
    unimplemented!("as")
}

fn dimensiynau(maes_ffrwydron: &[&str]) -> (usize, usize) {
    (maes_ffrwydron.len(), maes_ffrwydron[0].len())
}

fn tarddiadau(maes_ffrwydron: &[&str]) -> Vec<(usize, usize)> {
    let (x_uchaf, y_uchaf) = dimensiynau(maes_ffrwydron);
    c![
        (x, y),
        for x in 0..x_uchaf,
        for y in 0..y_uchaf
    ]
}

fn cymdogion(maes_ffrwydron: &[&str], tarddiad: (usize, usize)) -> Vec<(usize, usize)> {
    let (tarddiad_x, tarddiad_y) = tarddiad;
    let (x_uchaf, y_uchaf) = dimensiynau(maes_ffrwydron);
    c![
        (x, y),
        for x in max(0, tarddiad_x - 1)..min(x_uchaf, tarddiad_x + 1),
        for y in max(0, tarddiad_y - 1)..min(y_uchaf, tarddiad_y + 1)
    ]
}

fn cynyddu_nod(nod: char) -> char {
    if nod == '*' {
        nod
    } else {
        (nod as u8 + 1) as char
    }
}

fn anodi_cell(maes_ffrwydron: &[&str], cell: (usize, usize)) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    out
}

fn anodi_cymdogion(maes_ffrwydron: &[&str], tarddiad: (usize, usize)) -> Vec<String> {}
