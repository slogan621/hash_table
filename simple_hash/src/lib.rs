#![allow(dead_code)]

use hash_table::{HashFn, HashTable, HashTableEntry, HashTableError};
use std::marker::PhantomData;

struct BasicHash<T, const N: usize> {
    data: Vec<HashTableEntry>,
    _phantom: PhantomData<T>,
}

impl<T, const N: usize> BasicHash<T, N> {
    fn get_capacity(&self) -> usize {
        self.data.capacity()
    }
}

struct BasicHashBuilder<T, const N: usize> {
    _phantom: PhantomData<T>,
}

impl<T, const N: usize> BasicHashBuilder<T, N> {
    fn new() -> BasicHashBuilder<T, N> {
        BasicHashBuilder::<T, N> {
            _phantom: PhantomData {},
        }
    }

    pub fn build(self) -> BasicHash<T, N> {
        let mut hash = BasicHash::<T, N> {
            data: Vec::with_capacity(N),
            _phantom: PhantomData {},
        };
        for i in 0..N {
            let h = HashTableEntry {valid: false};
            hash.data.push(h);
        }
        hash
    }
}

impl<T, const N: usize> HashTable<u32> for BasicHash<T, N> {
    fn insert(&mut self, key: u32, data: HashTableEntry) -> Result<(), HashTableError> {
        let x = self.hash(key);
        if self.data[x].valid == false {
            self.data[x] = data;
        } else {
            let mut y = x + 1;
            if y == N {
                y = 0;
            }
            let mut inserted = false;
            while inserted == false && y != x {
                if self.data[y].valid == false {
                    self.data[x] = data;
                    inserted = true;
                    break;
                } else {
                    y = y + 1;
                    if y == N {
                        y = 0;
                    }
                }
            }
            if inserted == false {
                panic!("Item {x} was occupied and there are no free slots");
            }
        }
        Ok(())
    }
    fn delete(&mut self, key: u32) -> Result<(), HashTableError> {
        todo!()
    }
    fn lookup(&self, key: u32) -> Result<(), HashTableError> {
        todo!()
    }
}

impl<T, const N: usize> HashFn<u32> for BasicHash<T, N> {
    fn hash(&self, key: u32) -> usize {
        (key % N as u32) as usize
    }
}

impl<T, const N: usize> HashTable<u64> for BasicHash<T, N> {
    fn insert(&mut self, key: u64, data: HashTableEntry) -> Result<(), HashTableError> {
        todo!()
    }
    fn delete(&mut self, key: u64) -> Result<(), HashTableError> {
        todo!()
    }
    fn lookup(&self, key: u64) -> Result<(), HashTableError> {
        todo!()
    }
}

impl<T, const N: usize> HashFn<u64> for BasicHash<T, N> {
    fn hash(&self, key: u64) -> usize {
        (key % N as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_basic_hash() {
        let mut x = BasicHashBuilder::<u32, 10>::new().build();
        assert_eq!(x.get_capacity(), 10);
        assert_eq!(x.hash(45u32), 5);
        let item = Box::new(9999u16);
        let data: HashTableEntry = HashTableEntry { valid: true };
        assert!(x.insert(45u32, data).is_ok());
        let data: HashTableEntry = HashTableEntry { valid: true };
        assert!(x.insert(45u32, data).is_ok());
    }
}
