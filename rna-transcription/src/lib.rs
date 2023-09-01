const DNA_TO_RNA: [(char, char); 4] = [('G', 'C'), ('C', 'G'), ('T', 'A'), ('A', 'U')];

#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        to_rna_or_dna(dna, valid_dna).map(Dna)
    }

    pub fn into_rna(self) -> Rna {
        Rna(self
            .0
            .chars()
            .map(|c| {
                DNA_TO_RNA
                    .into_iter()
                    .find(|(dna, _rna)| dna == &c)
                    .unwrap()
                    .1
            })
            .collect::<String>())
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        to_rna_or_dna(rna, valid_rna).map(Rna)
    }
}

fn valid_dna(c: char) -> bool {
    DNA_TO_RNA.into_iter().any(|(dna, _rna)| dna == c)
}

fn valid_rna(c: char) -> bool {
    DNA_TO_RNA.into_iter().any(|(_dna, rna)| rna == c)
}

fn to_rna_or_dna<F>(sequence: &str, f: F) -> Result<String, usize>
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
