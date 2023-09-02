/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        match Self::is_palindrome(value, 0) {
            true => Some(Self(value)),
            false => None,
        }
    }

    fn is_palindrome(head: u64, tail: u64) -> bool {
        if head > tail {
            Self::is_palindrome(head / 10, tail * 10 + head % 10)
        } else {
            head == tail || head == tail / 10
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    unimplemented!(
        "returns the minimum and maximum number of palindromes of the products of two factors in the range {min} to {max}"
    );
}
