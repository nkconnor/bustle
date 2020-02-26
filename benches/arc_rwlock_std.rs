use bustle::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
struct Std<K>(Arc<RwLock<HashMap<K, ()>>>);

impl<K> BenchmarkTarget for Std<K>
where
    K: Send + Sync + From<u64> + Copy + 'static + Hash + Eq + std::fmt::Debug,
{
    type Key = K;

    fn with_capacity(capacity: usize) -> Self {
        Self(Arc::new(RwLock::new(HashMap::with_capacity(capacity))))
    }

    fn get(&mut self, key: &Self::Key) -> bool {
        self.0.read().unwrap().get(key).is_some()
    }

    fn insert(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().insert(*key, ()).is_some()
    }

    fn remove(&mut self, key: &Self::Key) -> bool {
        self.0.write().unwrap().remove(key).is_some()
    }

    fn update(&mut self, key: &Self::Key) -> bool {
        use std::collections::hash_map::Entry;
        let mut map = self.0.write().unwrap();
        if let Entry::Occupied(mut e) = map.entry(*key) {
            e.insert(());
            true
        } else {
            false
        }
    }
}

fn main() {
    Workload::new(1, Mix::read_heavy()).run::<Std<u64>>();
}