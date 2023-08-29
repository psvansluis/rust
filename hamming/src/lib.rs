/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    match s1.len() == s2.len() {
        false => None,
        true => Some(
            s1.chars()
                .zip(s2.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count(),
        ),
    }
}
//// alternative solution
// pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
//     let mut dist: usize = 0;
//     let mut i1 = s1.chars().into_iter();
//     let mut i2 = s2.chars().into_iter();
//     loop {
//         match (i1.next(), i2.next()) {
//             (Some(c1), Some(c2)) => {
//                 if c1 != c2 {
//                     dist += 1;
//                 }
//             }
//             (None, None) => return Some(dist),
//             _ => return None,
//         }
//     }
// }
