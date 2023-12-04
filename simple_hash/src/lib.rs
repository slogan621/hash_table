#![allow(dead_code)]

use hash_table::{HashFn, HashTable, HashTableEntry, HashTableError};
use std::marker::PhantomData;

struct BasicHash<T, U, const N: usize> {
    data: Vec<HashTableEntry<U>>,
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

impl<T, U: Default, const N: usize> BasicHashBuilder<T, U, N> {
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
            let h = HashTableEntry::<U>::default();
            hash.data.push(h);
        }
        hash
    }
}

impl<T, U, const N: usize> HashTable<u32, U> for BasicHash<T, U, N> {
    fn insert(&mut self, key: u32, data: HashTableEntry<U>) -> Result<(), HashTableError> {
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

impl<T, U, const N: usize> HashFn<u32> for BasicHash<T, U, N> {
    fn hash(&self, key: u32) -> usize {
        (key % N as u32) as usize
    }
}

impl<T, U, const N: usize> HashTable<u64, U> for BasicHash<T, U, N> {
    fn insert(&mut self, key: u64, data: HashTableEntry<U>) -> Result<(), HashTableError> {
        todo!()
    }
    fn delete(&mut self, key: u64) -> Result<(), HashTableError> {
        todo!()
    }
    fn lookup(&self, key: u64) -> Result<(), HashTableError> {
        todo!()
    }
}

impl<T, U, const N: usize> HashFn<u64> for BasicHash<T, U, N> {
    fn hash(&self, key: u64) -> usize {
        (key % N as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_basic_hash() {
        let mut x = BasicHashBuilder::<u32, u16, 10>::new().build();
        assert_eq!(x.get_capacity(), 10);
        assert_eq!(x.hash(45u32), 5);
        let item = Box::new(9999u16);
        let data: HashTableEntry<u16> = HashTableEntry::<u16> {
            valid: true,
            data: item.clone(),
        };
        assert!(x.insert(45u32, data).is_ok());
        let data: HashTableEntry<u16> = HashTableEntry::<u16> {
            valid: true,
            data: item.clone(),
        };
        assert!(x.insert(45u32, data).is_ok());
    }
}
