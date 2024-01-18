#![allow(dead_code)]

use hash_table::{HashFn, HashTable, HashTableEntry, HashTableError};
use rand::Rng;
use std::marker::PhantomData;

use std::mem;

const DEFAULT_STASH_SIZE: u16 = 8;
const MAX_SECONDARY_HASH_ITERATIONS: u16 = 8;

struct CuckooHash<U>
where
    U: Copy,
{
    primary: Vec<HashTableEntry<U>>,
    secondary: Vec<HashTableEntry<U>>,
    stash: Vec<HashTableEntry<U>>,
    // capacity of primary and secondary
    hash_capacity: u16,
    // capacity of stash
    stash_capacity: u16,
    // random number for hash of primary
    primary_random: u16,
    // random number for hash of secondary
    secondary_random: u16,
}

impl<U: std::marker::Copy> CuckooHash<U> {
    fn get_hash_capacity(&self) -> u16 {
        self.hash_capacity
    }
    fn get_stash_capacity(&self) -> u16 {
        self.stash_capacity
    }

    pub fn update_primary_random(&mut self) {
        self.primary_random = rand::thread_rng().gen::<u16>();
    }

    pub fn update_secondary_random(&mut self) {
        self.secondary_random = rand::thread_rng().gen::<u16>();
    }

    pub fn get_primary_random(&self) -> u16 {
        self.primary_random
    }

    pub fn get_secondary_random(&self) -> u16 {
        self.secondary_random
    }
}

struct CuckooHashBuilder<U> {
    _phantom_u: PhantomData<U>,
    hash_capacity: u16,
    stash_capacity: u16,
    primary_random: u16,
    secondary_random: u16,
}

impl<U: Default + std::marker::Copy> CuckooHashBuilder<U> {
    fn new() -> CuckooHashBuilder<U> {
        CuckooHashBuilder::<U> {
            _phantom_u: PhantomData {},
            hash_capacity: 0,
            stash_capacity: DEFAULT_STASH_SIZE,
            primary_random: rand::thread_rng().gen::<u16>(),
            secondary_random: rand::thread_rng().gen::<u16>(),
        }
    }

    pub fn with_hash_capacity(mut self, capacity: u16) -> CuckooHashBuilder<U> {
        self.hash_capacity = capacity;
        self
    }

    pub fn with_stash_capacity(mut self, capacity: u16) -> CuckooHashBuilder<U> {
        self.stash_capacity = capacity;
        self
    }

    pub fn build(self) -> CuckooHash<U> {
        let mut hash = CuckooHash::<U> {
            primary: Vec::with_capacity(self.hash_capacity.into()),
            secondary: Vec::with_capacity(self.hash_capacity.into()),
            stash: Vec::with_capacity(self.stash_capacity.into()),
            hash_capacity: self.hash_capacity,
            stash_capacity: self.stash_capacity,
            primary_random: 0,
            secondary_random: 0,
        };

        // initialize the primary and secondary hash tables
        for _i in 0..self.hash_capacity {
            let h = HashTableEntry::<U>::default();
            hash.primary.push(h);
            let h = HashTableEntry::<U>::default();
            hash.secondary.push(h);
        }

        // initialize the stash
        for _i in 0..self.stash_capacity {
            let h = HashTableEntry::<U>::default();
            hash.stash.push(h);
        }

        hash.update_primary_random();
        hash.update_secondary_random();

        hash
    }
}

impl<U: Default + std::marker::Copy + std::fmt::Debug> HashTable<U> for CuckooHash<U> {
    fn insert(&mut self, key: u16, data: U) -> Result<(), HashTableError> {
        let ret = self.lookup(key);
        if ret.is_ok() {
            return Ok(());
        }
        let mut p_key = key;
        let mut p_data = data;
        for _ in 0..MAX_SECONDARY_HASH_ITERATIONS {
            let h1: usize = self.hash(p_key).into();
            if self.primary[h1].data.is_none() {
                self.primary[h1].key = key;
                self.primary[h1].data = Some(Box::new(p_data));
                return Ok(());
            }
            let temp_key = self.primary[h1].key;
            let temp_data = **self.primary[h1].data.as_ref().unwrap();
            self.primary[h1].key = p_key;
            self.primary[h1].data = Some(Box::new(p_data));
            p_key = temp_key;
            p_data = temp_data;
            let h2: usize = self.secondary_hash(p_key).into();
            if self.secondary[h2].data.is_none() {
                self.secondary[h2].key = p_key;
                self.secondary[h2].data = Some(Box::new(p_data));
                return Ok(());
            }
            let temp_key = self.secondary[h2].key;
            let temp_data = **self.secondary[h2].data.as_ref().unwrap();
            self.secondary[h2].key = p_key;
            self.secondary[h2].data = Some(Box::new(p_data));
            p_key = temp_key;
            p_data = temp_data;
        }
        self.rehash()?;
        self.insert(p_key, p_data)?;
        Ok(())
    }

    /// delete means finding the value in either primary or
    /// secondary tables and clearing it, which is O(1)
    fn delete(&mut self, key: u16) -> Result<(), HashTableError> {
        let x: usize = self.hash(key).into();
        if self.primary[x].data.is_none() || self.primary[x].key != key {
            let x: usize = self.secondary_hash(key).into();
            if self.secondary[x].data.is_none() || self.secondary[x].key != key {
                // not found
                return Ok(());
            } else {
                let h = HashTableEntry::<U>::default();
                self.secondary[x] = h;
                return Ok(());
            }
        } else {
            let h = HashTableEntry::<U>::default();
            self.primary[x] = h;
            return Ok(());
        }
    }

    /// lookup means finding the value in either primary or
    /// secondary tables and is O(1)
    fn lookup(&self, key: u16) -> Result<U, HashTableError>
    where
        U: Copy,
    {
        let x: usize = self.hash(key).into();
        if self.primary[x].data.is_none() || self.primary[x].key != key {
            let x: usize = self.secondary_hash(key).into();
            if self.secondary[x].data.is_none() || self.secondary[x].key != key {
                return Err(HashTableError::NotFound);
            } else {
                return Ok(**self.secondary[x].data.as_ref().unwrap());
            }
        } else {
            return Ok(**self.primary[x].data.as_ref().unwrap());
        }
    }
}

impl<U: std::marker::Copy> HashFn for CuckooHash<U> {
    fn hash(&self, key: u16) -> u16 {
        (key & self.get_primary_random()) % self.get_hash_capacity()
    }
}

impl<U: Default + std::fmt::Debug + std::marker::Copy> CuckooHash<U> {
    fn initialize_hash_tables(&mut self) {
        // initialize the primary and secondary hash tables
        for _i in 0..self.hash_capacity {
            let h = HashTableEntry::<U>::default();
            self.primary.push(h);
            let h = HashTableEntry::<U>::default();
            self.secondary.push(h);
        }
    }
    // implementation of a second hash function, not a part of the
    // hash table trait, but needed to get a different hash result
    // across the secondary hash table
    fn secondary_hash(&self, key: u16) -> u16 {
        (key & self.get_secondary_random()) % self.get_hash_capacity()
    }

    /// rehash the primary and secondary tables with new hash functions
    fn rehash(&mut self) -> Result<(), HashTableError> {
        // a new hash function is defined as new random values for each
        // table (hash fns in this impl are computed using prime table size
        // and a random number.

        // create new hash functions by generating new random number for each
        self.update_primary_random();
        self.update_secondary_random();

        // get all entries to rehash

        let p = mem::take(&mut self.primary);
        let s = mem::take(&mut self.secondary);

        self.initialize_hash_tables();

        // iterate the list of entries and re-add them to the hash table

        // unwrap is safe in below code because we have Some(_)

        for mut val in p {
            match val.data {
                Some(_) => { 
                    self.insert(val.key, *mem::take(&mut val.data).unwrap())?;
                },
                _ => {},
            }
        }

        for mut val in s {
            match val.data {
                Some(_) => { 
                    self.insert(val.key, *mem::take(&mut val.data).unwrap())?;
                },
                _ => {},
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_cuckoo_hash() {
        let mut x = CuckooHashBuilder::<u16>::new()
            .with_hash_capacity(3)
            .build();
        assert_eq!(x.get_hash_capacity(), 3);
        let mut item = 10;
        assert!(x.insert(1, item).is_ok());
        item += 10;
        assert!(x.insert(2, item).is_ok());
        item += 10;
        assert!(x.insert(3, item).is_ok());
        item += 10;
        assert!(x.insert(4, item).is_ok());
    }

    #[test]
    fn can_create_cuckoo_hash_and_search() {
        let mut x = CuckooHashBuilder::<u16>::new()
            .with_hash_capacity(3)
            .build();

        assert_eq!(x.get_hash_capacity(), 3);
        let mut item = 10;
        assert!(x.insert(1, item).is_ok());
        item += 10;
        assert!(x.insert(2, item).is_ok());
        item += 10;
        assert!(x.insert(3, item).is_ok());
        item += 10;
        assert!(x.insert(4, item).is_ok());

        let ret = x.lookup(1);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 10);
        let ret = x.lookup(2);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 20);
        let ret = x.lookup(3);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 30);
        let ret = x.lookup(4);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 40);
        let ret = x.lookup(17);
        assert_eq!(ret.is_ok(), false);
    }

    #[test]
    fn can_create_cuckoo_hash_and_delete() {
        let mut x = CuckooHashBuilder::<u16>::new()
            .with_hash_capacity(3)
            .build();
        assert_eq!(x.get_hash_capacity(), 3);
        let mut item = 10;
        assert!(x.insert(1, item).is_ok());
        item += 10;
        assert!(x.insert(2, item).is_ok());
        item += 10;
        assert!(x.insert(3, item).is_ok());
        item += 10;
        assert!(x.insert(4, item).is_ok());
        item += 10;
        assert!(x.insert(5, item).is_ok());
        item += 10;
        assert!(x.insert(6, item).is_ok());

        let ret = x.lookup(1);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 10);
        let ret = x.lookup(2);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 20);
        let ret = x.lookup(3);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 30);
        let ret = x.lookup(4);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 40);
        let ret = x.lookup(5);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 50);
        let ret = x.lookup(6);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 60);

        let ret = x.delete(1);
        assert!(ret.is_ok());
        let ret = x.lookup(1);
        assert_eq!(ret.is_ok(), false);
        let ret = x.delete(2);
        assert!(ret.is_ok());
        let ret = x.lookup(2);
        assert_eq!(ret.is_ok(), false);
        let ret = x.delete(3);
        assert!(ret.is_ok());
        let ret = x.lookup(3);
        assert_eq!(ret.is_ok(), false);
        let ret = x.delete(4);
        assert!(ret.is_ok());
        let ret = x.lookup(4);
        assert_eq!(ret.is_ok(), false);
        let ret = x.delete(5);
        assert!(ret.is_ok());
        let ret = x.lookup(5);
        assert_eq!(ret.is_ok(), false);
        let ret = x.delete(6);
        assert!(ret.is_ok());
        let ret = x.lookup(6);
        assert_eq!(ret.is_ok(), false);

        // see if we can insert them again

        let mut item = 100;
        assert!(x.insert(1, item).is_ok());
        item += 100;
        assert!(x.insert(2, item).is_ok());
        item += 100;
        assert!(x.insert(3, item).is_ok());

        let ret = x.lookup(1);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 100);
        let ret = x.lookup(2);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 200);
        let ret = x.lookup(3);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 300);

        let ret = x.delete(1);
        assert!(ret.is_ok());
        let ret = x.lookup(1);
        assert_eq!(ret.is_ok(), false);
        let ret = x.delete(2);
        assert!(ret.is_ok());
        let ret = x.lookup(2);
        assert_eq!(ret.is_ok(), false);
        let ret = x.delete(3);
        assert!(ret.is_ok());
        let ret = x.lookup(3);
        assert_eq!(ret.is_ok(), false);
    }
}
