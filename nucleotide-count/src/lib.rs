use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let valid_nucleotides = ['A', 'C', 'G', 'T'];
    if !valid_nucleotides.contains(&nucleotide) {
        return Err(nucleotide);
    }

    let mut count = 0;
    for ch in dna.chars() {
        if !valid_nucleotides.contains(&ch) {
            return Err(ch);
        }

        if ch == nucleotide {
            count += 1;
        }
    }

    Ok(count)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let valid_nucleotides = ['A', 'C', 'G', 'T'];

    let mut map: HashMap<char, usize> = HashMap::new();
    for nucleotide in valid_nucleotides.iter() {
        match count(*nucleotide, dna) {
            Ok(count) => {
                map.insert(*nucleotide, count);
            }
            Err(char) => {
                return Err(char);
            }
        }
    }
    return Ok(map);
}
