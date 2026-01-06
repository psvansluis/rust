const LINE_HEIGHT: usize = 4;
const LINE_WIDTH: usize = 3;

const H: u8 = b'_';
const V: u8 = b'|';
const S: u8 = b' ';

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines = input.lines().collect::<Vec<&str>>();

    let (chunked_lines, []) = lines.as_chunks::<LINE_HEIGHT>() else {
        return Result::Err(Error::InvalidRowCount(lines.len()));
    };

    return chunked_lines
        .iter()
        .map(convert_line)
        .collect::<Result<Vec<String>, Error>>()
        .map(|lines| lines.join(","));
}

fn convert_line(input: &[&str; LINE_HEIGHT]) -> Result<String, Error> {
    let mut x = Vec::new();

    for line in input {
        let (chunks, remainder) = line.as_bytes().as_chunks::<LINE_WIDTH>();

        if !remainder.is_empty() {
            return Err(Error::InvalidColumnCount(line.len()));
        }

        x.push(chunks);
    }

    let mut y = String::new();

    for (i, _) in x[0].iter().enumerate() {
        y.push(convert_char(&[x[0][i], x[1][i], x[2][i], x[3][i]]));
    }

    return Ok(y);
}

fn convert_char(input: &[[u8; LINE_WIDTH]; LINE_HEIGHT]) -> char {
    match input {
        [[S, H, S], [V, S, V], [V, H, V], [S, S, S]] => '0',
        [[S, S, S], [S, S, V], [S, S, V], [S, S, S]] => '1',
        [[S, H, S], [S, H, V], [V, H, S], [S, S, S]] => '2',
        [[S, H, S], [S, H, V], [S, H, V], [S, S, S]] => '3',
        [[S, S, S], [V, H, V], [S, S, V], [S, S, S]] => '4',
        [[S, H, S], [V, H, S], [S, H, V], [S, S, S]] => '5',
        [[S, H, S], [V, H, S], [V, H, V], [S, S, S]] => '6',
        [[S, H, S], [S, S, V], [S, S, V], [S, S, S]] => '7',
        [[S, H, S], [V, H, V], [V, H, V], [S, S, S]] => '8',
        [[S, H, S], [V, H, V], [S, H, V], [S, S, S]] => '9',
        _ => '?',
    }
}
