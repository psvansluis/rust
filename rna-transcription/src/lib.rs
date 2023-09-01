const DNA_TO_RNA: [(char, char); 4] = [('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        string_if_all_or_index(dna, valid_dna).map(Dna)
    }

    pub fn into_rna(self) -> Rna {
        self.0
            .chars()
            .map(dna_to_rna)
            .collect::<Option<String>>()
            .map(Rna)
            .unwrap()
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        string_if_all_or_index(rna, valid_rna).map(Rna)
    }
}

fn dna_to_rna(c: char) -> Option<char> {
    DNA_TO_RNA
        .into_iter()
        .find(|(dna, _rna)| dna == &c)
        .map(|t| t.1)
}

fn valid_dna(c: char) -> bool {
    DNA_TO_RNA.into_iter().any(|(dna, _rna)| dna == c)
}

fn valid_rna(c: char) -> bool {
    DNA_TO_RNA.into_iter().any(|(_dna, rna)| rna == c)
}

fn string_if_all_or_index<F>(sequence: &str, f: F) -> Result<String, usize>
where
    F: Fn(char) -> bool,
{
    sequence
        .chars()
        .enumerate()
        .map(|(i, c)| match f(c) {
            true => Ok(c),
            _ => Err(i),
        })
        .collect::<Result<String, usize>>()
}
