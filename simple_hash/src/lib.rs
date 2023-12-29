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

impl<U: std::marker::Copy + std::fmt::Debug> HashTable<U> for BasicHash<U> {
    fn insert(&mut self, key: u16, data: U) -> Result<(), HashTableError> {
        let x : usize = self.hash(key.clone()).into();
        if self.data[x].valid == false {
            self.data[x].key = key.clone();
            self.data[x].valid = true;
            self.data[x].data = Some(Box::new(data));
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
                    self.data[y].data = Some(Box::new(data));
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
        let x : usize = self.hash(key.clone()).into();
        if self.data[x].valid == false {
            return Err(HashTableError::NotFound);
        }
        if self.data[x].key == key {
            self.data[x].valid = false;
            self.data[x].data = None;
            return Ok(());
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
                self.data[x].valid = false;
                self.data[x].data = None;
                return Ok(());
            }
        }
        return Err(HashTableError::NotFound);
    }
    fn lookup(&self, key: u16) -> Result<U, HashTableError> 
        where U : Copy {
        let x : usize = self.hash(key.clone()).into();
        if self.data[x].valid == false {
            return Err(HashTableError::NotFound);
        }
        if self.data[x].key == key {
            return Ok(**self.data[x].data.as_ref().unwrap());
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
                return Ok(**self.data[y].data.as_ref().unwrap());
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
        
    #[test]
    fn can_create_basic_hash_and_delete() {
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

        let ret = x.delete(1);
        assert!(ret.is_ok());
        let ret = x.lookup(1);
        assert_eq!(ret.is_ok(), false);
        let ret = x.lookup(2);
        assert_eq!(ret.is_ok(), true);
        let ret = x.lookup(3);
        assert_eq!(ret.is_ok(), true);
        let ret = x.delete(2);
        assert!(ret.is_ok());
        let ret = x.lookup(2);
        assert_eq!(ret.is_ok(), false);
        let ret = x.lookup(3);
        assert_eq!(ret.is_ok(), true);
        let ret = x.delete(3);
        assert!(ret.is_ok());
        let ret = x.lookup(3);
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
        let ret = x.lookup(2);
        assert_eq!(ret.is_ok(), true);
        let ret = x.lookup(3);
        assert_eq!(ret.is_ok(), true);
        let ret = x.delete(2);
        assert!(ret.is_ok());
        let ret = x.lookup(2);
        assert_eq!(ret.is_ok(), false);
        let ret = x.lookup(3);
        assert_eq!(ret.is_ok(), true);
        let ret = x.delete(3);
        assert!(ret.is_ok());
        let ret = x.lookup(3);
        assert_eq!(ret.is_ok(), false);
    }
}
