pub struct MultiSet<K: Ord + Clone> {
    map: std::collections::BTreeMap<K, usize>,
    counter: usize,
}

impl<K: Ord + Clone> MultiSet<K> {
    pub fn new() -> Self {
        Self {
            map: std::collections::BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn insert(&mut self, k: K) {
        *self.map.entry(k).or_insert(0) += 1;
        self.counter += 1;
    }

    pub fn remove_one(&mut self, k: &K) -> bool {
        if !self.contains(k) {
            false
        } else {
            self.counter -= 1;
            if self.map[k] == 1 {
                self.map.remove(k);
            } else {
                let v = self.map[k];
                self.map.insert(k.clone(), v - 1);
            }
            true
        }
    }

    pub fn remove_all(&mut self, k: &K) -> bool {
        if !self.contains(k) {
            false
        } else {
            let v = self.map[k];
            self.counter -= v;
            self.map.remove(k);
            true
        }
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.counter = 0;
    }

    pub fn contains(&self, k: &K) -> bool {
        self.map.contains_key(k)
    }

    pub fn len(&self) -> usize {
        self.counter
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn first(&self) -> Option<&K> {
        self.map.iter().next().map(|o| o.0)
    }

    pub fn last(&self) -> Option<&K> {
        self.map.iter().next_back().map(|o| o.0)
    }

    pub fn pop_first(&mut self) -> Option<K> {
        if let Some(k) = self.first().map(|o| o.clone()) {
            self.remove_one(&k);
            Some(k)
        } else {
            None
        }
    }

    pub fn pop_last(&mut self) -> Option<K> {
        if let Some(k) = self.last().map(|o| o.clone()) {
            self.remove_one(&k);
            Some(k)
        } else {
            None
        }
    }
}

impl<K: Ord + Clone> IntoIterator for MultiSet<K> {
    type Item = K;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = Vec::new();
        self.map.into_iter().for_each(|(k, v)| {
            for _ in 0..v {
                vec.push(k.clone());
            }
        });
        vec.into_iter()
    }
}
