#![allow(dead_code)]

use hash_table::{HashFn, HashTable, HashTableEntry, HashTableError};
use std::marker::PhantomData;
use std::ops::Rem;

struct BasicHash<T, U, const N: usize> {
    data: Vec<HashTableEntry<T, U>>,
    _phantom: PhantomData<T>,
}

impl<T, U, const N: usize> BasicHash<T, U, N> {
    fn get_capacity(&self) -> usize {
        self.data.capacity()
    }
}

struct BasicHashBuilder<T, U, const N: usize> {
    _phantom: PhantomData<T>,
    _phantom_u: PhantomData<U>,
}

impl<T: Default, U: Default, const N: usize> BasicHashBuilder<T, U, N> {
    fn new() -> BasicHashBuilder<T, U, N> {
        BasicHashBuilder::<T, U, N> {
            _phantom: PhantomData {},
            _phantom_u: PhantomData {},
        }
    }

    pub fn build(self) -> BasicHash<T, U, N> {
        let mut hash = BasicHash::<T, U, N> {
            data: Vec::with_capacity(N),
            _phantom: PhantomData {},
        };

        // initialize the hash table
        for _i in 0..N {
            let h = HashTableEntry::<T, U>::default();
            hash.data.push(h);
        }
        hash
    }
}

impl<T: Clone, U, const N: usize> HashTable<T, U> for BasicHash<T, U, N> {
    fn insert(&mut self, key: T, data: U) -> Result<(), HashTableError> {
        let x = self.hash(key.clone());
        if self.data[x].valid == false {
            self.data[x].key = key.clone();
            self.data[x].valid = true;
            self.data[x].data = Box::new(data);
        } else {
            let mut y = x + 1;
            if y == N {
                y = 0;
            }
            let mut inserted = false;
            while inserted == false && y != x {
                if self.data[y].valid == false {
                    self.data[y].key = key.clone();
                    self.data[y].valid = true;
                    self.data[y].data = Box::new(data);
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
    fn delete(&mut self, key: T) -> Result<(), HashTableError> {
        todo!()
    }
    fn lookup(&self, key: T) -> Result<(), HashTableError> {
        todo!()
    }
}

//impl<T, U, const N: usize> HashFn<T> for BasicHash<T, U, N> where T: std::ops::Rem<usize, Output = usize> {
impl<T, U, const N: usize> HashFn<T> for BasicHash<T, U, N> {
    fn hash(&self, key: T) -> usize {
        unsafe { key % N }
    }
}

/*

impl<T, U, const N: usize> HashTable<u64, U> for BasicHash<T, U, N> {
/*
    fn insert(&mut self, key: u64, data: HashTableEntry<u64, U>) -> Result<(), HashTableError> {
        todo!()
    }
*/
    fn insert(&mut self, key: T, data: U) -> Result<(), HashTableError> {
        todo!()
    }
    fn delete(&mut self, key: T) -> Result<(), HashTableError> {
        todo!()
    }
    fn lookup(&self, key: T) -> Result<(), HashTableError> {
        todo!()
    }
}

impl<T, U, const N: usize> HashFn<u64> for BasicHash<T, U, N> {
    fn hash(&self, key: u64) -> usize {
        (key % N as u64) as usize
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_basic_hash() {
        let mut x = BasicHashBuilder::<u32, u16, 10>::new().build();
        assert_eq!(x.get_capacity(), 10);
        assert_eq!(x.hash(45u32), 5);
        let item = 9999u16;
        assert!(x.insert(45u32, item).is_ok());
/*
        let data: HashTableEntry<u32, u16> = HashTableEntry::<u32, u16> {
            key: 0u32,
            valid: true,
            data: item.clone(),
        };
        assert!(x.insert(45u32, data).is_ok());
        let data: HashTableEntry<u32, u16> = HashTableEntry::<u32, u16> {
            key: 0u32,
            valid: true,
            data: item.clone(),
        };
        assert!(x.insert(45u32, data).is_ok());
*/
    }
}
