const BASE: u64 = 10;

/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        match value % BASE > 0 && {
            let (head, tail) = Self::head_and_tail(value, 0);
            head == tail
        } {
            true => Some(Self(value)),
            false => None,
        }
    }

    fn head_and_tail(head: u64, tail: u64) -> (u64, u64) {
        if head > tail {
            Self::head_and_tail(head / BASE, tail * BASE + head % BASE)
        } else {
            (head, tail / if tail > head * BASE { BASE } else { 1 })
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes: Vec<Palindrome> = (min..=max)
        .map(|n| (n..=max).map(move |m| n * m))
        .flatten()
        .filter_map(Palindrome::new)
        .collect();
    palindromes.sort_unstable();
    match palindromes.len() {
        0 => None,
        len => Some((palindromes[0], palindromes[len - 1])),
    }
}
