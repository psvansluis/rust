use std::collections::HashMap;
const STOP: &Option<&str> = &Some("stop codon");

pub struct CodonsInfo<'a>(HashMap<&'a str, &'a str>);

impl<'a> CodonsInfo<'a> {
    pub fn name_for(&self, codon: &str) -> Option<&'a str> {
        self.0.get(codon).copied()
    }

    pub fn of_rna(&self, rna: &str) -> Option<Vec<&'a str>> {
        rna.as_bytes()
            .chunks(3)
            .map(std::str::from_utf8)
            .map(Result::ok)
            .map(|codon| codon.map(|c| self.name_for(c)))
            .map(Option::flatten)
            .take_while(|protein| protein != STOP)
            .collect()
    }
}

pub fn parse<'a>(pairs: Vec<(&'a str, &'a str)>) -> CodonsInfo<'a> {
    CodonsInfo(pairs.into_iter().collect())
}
