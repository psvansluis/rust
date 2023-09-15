use once_cell::sync::Lazy;
use rand::{rngs::ThreadRng, thread_rng, Rng};
use std::{collections::HashSet, sync::Mutex};

static NAMES: Lazy<Mutex<HashSet<String>>> = Lazy::new(|| Mutex::new(HashSet::<String>::new()));

fn repo() -> std::sync::MutexGuard<'static, HashSet<String>> {
    NAMES.lock().unwrap()
}
pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Self(Self::generate_unique_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        let new_name = Self::generate_unique_name();
        repo().remove(&self.0);
        self.0 = new_name;
    }

    fn generate_unique_name() -> String {
        let name = Self::generate_name();
        if repo().insert(name.clone()) {
            name
        } else {
            Self::generate_unique_name()
        }
    }

    fn generate_name() -> String {
        let rng = &mut thread_rng();
        let ch = |rng: &mut ThreadRng| rng.gen_range('A'..='Z');
        format!("{}{}{:03}", ch(rng), ch(rng), rng.gen_range(0..1000))
    }
}
