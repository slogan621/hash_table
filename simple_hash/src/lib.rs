#![allow(dead_code)]

use hash_table::{HashFn, HashTable, HashTableEntry, HashTableError};
use std::marker::PhantomData;

struct BasicHash<U> {
    data: Vec<HashTableEntry<U>>,
    capacity: u16,
}

impl<U> BasicHash<U> {
    fn get_capacity(&self) -> u16 {
        self.capacity
    }
}

struct BasicHashBuilder<U> {
    _phantom_u: PhantomData<U>,
    capacity: u16,
}

impl<U: Default> BasicHashBuilder<U> {
    fn new() -> BasicHashBuilder<U> {
        BasicHashBuilder::<U> {
            _phantom_u: PhantomData {},
            capacity: 0,
        }
    }

    pub fn with_capacity(mut self, capacity: u16) -> BasicHashBuilder<U> {
        self.capacity = capacity;
        self
    }

    pub fn build(self) -> BasicHash<U> {
        let mut hash = BasicHash::<U> {
            data: Vec::with_capacity(self.capacity.into()),
            capacity: self.capacity,
        };

        // initialize the hash table
        for _i in 0..self.capacity {
            let h = HashTableEntry::<U>::default();
            hash.data.push(h);
        }
        hash
    }
}

impl<U: std::marker::Copy> HashTable<U> for BasicHash<U> {
    fn insert(&mut self, key: u16, data: U) -> Result<(), HashTableError> {
        let x : usize = self.hash(key.clone()).into();
        if self.data[x].valid == false {
            self.data[x].key = key.clone();
            self.data[x].valid = true;
            self.data[x].data = Box::new(data);
        } else {
            let mut y = x + 1;
            if y == self.get_capacity().into() {
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
                    if y == self.get_capacity().into() {
                        y = 0;
                    }
                }
            }
            if inserted == false {
                return Err(HashTableError::TableFull);
            }
        }
        Ok(())
    }
    fn delete(&mut self, key: u16) -> Result<(), HashTableError> {
        todo!()
    }
    fn lookup(&self, key: u16) -> Result<U, HashTableError> 
        where U : Copy {
        let x : usize = self.hash(key.clone()).into();
        if self.data[x].valid == false {
            return Err(HashTableError::NotFound);
        }
        if self.data[x].key == key {
            return Ok(*self.data[x].data);
        } else {
            let mut y = x+1;
            if y == self.get_capacity().into() {
                y = 0;
            }
            while y != x && self.data[x].valid && self.data[y].key != key {
                y += 1;
                if y == self.get_capacity().into() {
                    y = 0;
                }
            }
            if self.data[y].valid == true && self.data[y].key == key {
                return Ok(*self.data[y].data);
            }
        }
        return Err(HashTableError::NotFound);
    }
}

//impl<T, U, const N: T> HashFn<T> for BasicHash<T, U, N> where T: std::ops::Rem<usize, Output = usize> {
impl<U> HashFn for BasicHash<U> {
    fn hash(&self, key: u16) -> u16 {
        key % self.get_capacity()
    }
}

/*

impl<T, U, const N: T> HashTable<u64, U> for BasicHash<T, U, N> {
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

impl<T, U, const N: T> HashFn<u64> for BasicHash<T, U, N> {
    fn hash(&self, key: u64) -> T {
        (key % N as u64) as T
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_basic_hash() {
        let mut x = BasicHashBuilder::<u16>::new().with_capacity(3).build();
        assert_eq!(x.get_capacity(), 3);
        let mut item = 10;
        assert!(x.insert(1, item).is_ok());
        item += 10;
        assert!(x.insert(2, item).is_ok());
        item += 10;
        assert!(x.insert(3, item).is_ok());
        item += 10;
        assert_eq!(x.insert(4, item).is_ok(), false);
    }
        
    #[test]
    fn can_create_basic_hash_and_search() {
        let mut x = BasicHashBuilder::<u16>::new().with_capacity(3).build();
        assert_eq!(x.get_capacity(), 3);
        let mut item = 10;
        assert!(x.insert(1, item).is_ok());
        item += 10;
        assert!(x.insert(2, item).is_ok());
        item += 10;
        assert!(x.insert(3, item).is_ok());
    
        let ret = x.lookup(1);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 10);
        let ret = x.lookup(2);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 20);
        let ret = x.lookup(3);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), 30);
        let ret = x.lookup(17);
        assert_eq!(ret.is_ok(), false);
    }
        
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
