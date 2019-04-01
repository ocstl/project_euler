use std::collections::HashMap;
use std::collections::hash_map::{Iter, Keys, Values};
use std::hash::Hash;
use std::iter::FromIterator;

#[derive(Default)]
pub struct Counter<K: Eq + Hash> {
    map: HashMap<K, usize>,
}

impl<K: Eq + Hash> Counter<K> {
    pub fn new() -> Counter<K> {
        Counter { map: HashMap::new() }
    }

    pub fn add(&mut self, k: K) -> &mut Self {
        self.map.entry(k)
            .and_modify(|e| *e += 1)
            .or_insert(1);

        self
    }

    pub fn subtract(&mut self, k: K) -> &mut Self {
        match self.map.get_mut(&k) {
            None => (),
            Some(1) => { self.map.remove(&k); },
            Some(v) => *v -= 1,
        }

        self
    }

    pub fn get(&self, k: &K) -> usize {
        if let Some(&v) = self.map.get(k) {
            v
        } else {
            0
        }
    }

    pub fn keys(&self) -> Keys<K, usize> {
        self.map.keys()
    }

    pub fn values(&self) -> Values<K, usize> {
        self.map.values()
    }

    pub fn iter(&self) -> Iter<K, usize> {
        self.map.iter()
    }
}

impl<K: Eq + Hash> FromIterator<K> for Counter<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut counter = Counter::new();
        for i in iter {
            counter.add(i);
        }

        counter
    }
}