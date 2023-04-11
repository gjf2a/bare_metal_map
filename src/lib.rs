#![cfg_attr(not(test), no_std)]

#[derive(Copy, Clone, Debug)]
pub struct BareMetalMap<K:Copy, V:Copy, const MAX_ENTRIES: usize> {
    map: [Option<(K, V)>; MAX_ENTRIES]
}

impl<K:Copy+Clone+Default+Eq+PartialEq,V:Copy+Clone+Default,const MAX_ENTRIES: usize> Default for BareMetalMap<K, V, MAX_ENTRIES> {
    fn default() -> Self {
        Self { map: [None; MAX_ENTRIES] }
    }
}

impl<K:Copy+Clone+Default+Eq+PartialEq,V:Copy+Clone+Default,const MAX_ENTRIES: usize> BareMetalMap<K, V, MAX_ENTRIES> {
    pub fn new() -> Self {Self::default()}

    pub fn get(&self, key: K) -> Option<V> {
        for entry in self.map.iter() {
            if let Some((k, v)) = entry {
                if key == *k {
                    return Some(*v);
                }
            }
        }
        None
    }

    pub fn get_at(&self, i: usize) -> Option<V> {
        self.map[i].map(|(_, v)| v)
    }

    pub fn put(&mut self, key: K, value: V) -> Option<usize> {
        for i in 0..self.map.len() {
            if self.map[i].is_none() {
                self.map[i] = Some((key, value));
                return Some(i);
            }
        }
        None
    }

    pub fn for_each<F: Fn(K,V)>(&self, func: F) {
        self.map.iter().filter_map(|e| *e).for_each(|(k, v)| func(k, v));
    }

    pub fn for_each_mut<F: FnMut(K, V)>(&self, mut func: F) {
        self.map.iter().filter_map(|e| *e).for_each(|(k, v)| func(k, v));
    }

    pub fn len(&self) -> usize {
        self.map.iter().filter(|e| e.is_some()).count()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut map: BareMetalMap<&str, i32, 10> = BareMetalMap::new();
        map.put("hello", 0);
        map.put("bye", 1);
        map.put("greetings", 2);

        map.for_each(|k, v| println!("{k}: {v}"));

        let mut count = 0;
        map.for_each_mut(|_, _| {count += 1;});
        assert_eq!(count, 3);
    }
}
