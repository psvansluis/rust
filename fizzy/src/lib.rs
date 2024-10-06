// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use std::{fmt::Display, ops::Rem};

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    predicate: fn(T) -> bool,
    substitute: String,
}

impl<T> Matcher<T> {
    pub fn new<S: ToString>(predicate: fn(T) -> bool, substitute: S) -> Matcher<T> {
        Self {
            predicate,
            substitute: substitute.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
#[derive(Default)]
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T> Fizzy<T>
where
    T: Copy + ToString + Display,
{
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply(self, iter: impl Iterator<Item = T>) -> impl Iterator<Item = String> {
        iter.map(move |el| {
            match self
                .matchers
                .iter()
                .filter(|m| (m.predicate)(el))
                .map(|m| m.substitute.clone())
                .collect::<String>()
            {
                str if !str.is_empty() => str,
                _ => el.to_string(),
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Display + Copy + From<u8> + PartialEq + Rem<Output = T>,
{
    Fizzy {
        matchers: vec![
            Matcher::new(|n: T| n % 3.into() == 0.into(), "fizz"),
            Matcher::new(|n: T| n % 5.into() == 0.into(), "buzz"),
        ],
    }
}
