#[macro_export]
macro_rules! hashmap {
    ($($($key:expr => $val:expr)+$(,)?)*) => {{
        let mut hm = crate::HashMap::new();
        $($(hm.insert($key, $val);)*)*;
        hm
    }};
}
