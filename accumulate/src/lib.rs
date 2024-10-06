pub fn map<T, U, F>(input: Vec<T>, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    let init = Vec::with_capacity(input.len());
    input.into_iter().fold(init, |mut acc, el| {
        acc.push(function(el));
        acc
    })
}
