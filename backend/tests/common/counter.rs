use std::collections::HashMap;
use std::hash::Hash;

pub fn count<T: Hash + Eq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut counter = HashMap::new();

    for el in a {
        *counter.entry(el).or_insert(0) += 1;
    }

    for el in b {
        let entry = counter.entry(el).or_insert(0);
        if *entry == 0 {
            return false;
        }
        *entry -= 1;
    }

    true
}
