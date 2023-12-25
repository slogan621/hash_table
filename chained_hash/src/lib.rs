#![allow(dead_code)]

use hash_table::{HashFn, HashTable, HashTableEntry, HashTableError};
use std::marker::PhantomData;

use std::mem;

// below inspired by https://rust-unofficial.github.io/too-many-lists/first-final.html
// iterator impl is original code after some trial and error

type Link<U> = Option<Box<ChainedHashEntry<U>>>;

#[derive(Debug, PartialEq)]
pub struct List<U> {
    head: Link<U>,
}

#[derive(Debug, PartialEq)]
struct ChainedHashEntry<U> {
    data: HashTableEntry<U>,
    next: Link<U>,
}

struct ChainedHash<U: std::cmp::PartialEq> {
    table: Vec<List<U>>,
    capacity: u16,
}

#[derive(Debug)]
struct ChainedHashIterator<'a, U: std::cmp::PartialEq> {
    next: &'a Link<U>,
}

impl<'a, U: std::cmp::PartialEq + std::fmt::Debug> Iterator for ChainedHashIterator<'a, U> {
    type Item = &'a Link<U>;

    fn next(&mut self) -> Option<Self::Item> {

        let current = self.next;

        match &*current {
            None => {
                self.next = &None;
                None
            },
            Some(node) => {
                self.next = &node.next;
                Some(current)
            }
        }
    }
}

impl<U: std::cmp::PartialEq> List<U> {
    fn iter(&self) -> ChainedHashIterator<'_, U> {
        ChainedHashIterator {next: &self.head }
    }
}

impl<U: std::cmp::PartialEq> ChainedHash<U> {
    fn get_capacity(&self) -> u16 {
        self.capacity
    }
}

struct ChainedHashBuilder<U> {
    _phantom_u: PhantomData<U>,
    capacity: u16,
}

impl<U: Default + std::cmp::PartialEq> ChainedHashBuilder<U> {
    fn new() -> ChainedHashBuilder<U> {
        ChainedHashBuilder::<U> {
            _phantom_u: PhantomData {},
            capacity: 0,
        }
    }

    pub fn with_capacity(mut self, capacity: u16) -> ChainedHashBuilder<U> {
        self.capacity = capacity;
        self
    }

    pub fn build(self) -> ChainedHash<U> {
        let mut hash = ChainedHash::<U> {
            table: Vec::with_capacity(self.capacity.into()),
            capacity: self.capacity,
        };

        // initialize the hash table
        for _i in 0..self.capacity {
            hash.table.push(List { head: None });
        }
        hash
    }
}

impl<U: std::marker::Copy + std::fmt::Debug + std::cmp::PartialEq> HashTable<U> for ChainedHash<U> {
    fn insert(&mut self, key: u16, data: U) -> Result<(), HashTableError> {
        let x: usize = self.hash(key.clone()).into();

        let data = HashTableEntry::<U> {
            valid: true,
            key: key,
            data: Some(Box::new(data)),
        };

        let entry = ChainedHashEntry::<U> {
            data: data,
            next: mem::replace(&mut self.table[x].head, None),
        };
        self.table[x].head = Some(Box::new(entry));
        Ok(())
    }

    fn delete(&mut self, key: u16) -> Result<(), HashTableError> {
        let x : usize = self.hash(key.clone()).into();
        if self.table[x] == (List { head: None }) {
            return Err(HashTableError::NotFound);
        }

        let mut prev = None;
        let mut cur = &self.table[x].head;

        loop {
            match cur {
                Some(ref value) => {
                    println!("top of loop value is {:?}", value);
                    if value.data.key == key {
                        if prev == None.as_ref() {
                            // head of list
                            println!("deleting key {:?} from head of list",
                                key);
                            self.table[x].head = None;
                            //let _ = mem::replace(&self.table[x].head, Link::More(*value));
                        } else {
                            println!("deleting key {:?} from middle of list",
                                key);
                            println!("before replace prev {:?} cur {:?}",
prev, cur);
                            //let prev = mem::replace(&mut prev, &*cur);
                            let _ = mem::replace(&mut prev, Some(cur));
                            println!("after replace prev {:?} cur {:?}",
prev, cur);
                            println!("after replace table {:?}", self.table[x]);
                        }
                        return Ok(());
                    } else {
                        prev = Some(cur);
                        cur = &value.next;
                    }
                },
                _ => {}, // can't really happen
            }
        }

/*
        println!("before delete table {:?}", self.table[x]);
        //let mut cur_link = mem::replace(&mut self.table[x].head, None);
        let &mut cur_link = &mut self.table[x].head;
        let mut prev_link = &None;
        
        while let Some(ref boxed_node) = cur_link {
            //cur_link = mem::replace(&mut boxed_node.next, None);
            prev_link = &cur_link;
            cur_link = boxed_node.next
        }
        println!("after delete table {:?}", self.table[x]);
/*
        for elem in self.table[x].iter() {
            match elem {
                Link::More(value) => {
                    if value.data.key == key {
                        let _ = mem::replace(&mut prev, &value.next);
                        return Ok(());
                    } else {
                        prev = &value.next;
                    }
                },
                _ => {}, // can't really happen
            }
        }
*/
*/

        Err(HashTableError::NotFound)
    }
    fn lookup(&self, key: u16) -> Result<U, HashTableError>
    where
        U: Copy,
    {
        let x: usize = self.hash(key.clone()).into();

        if self.table[x] == (List { head: None }) {
            return Err(HashTableError::NotFound);
        }

        for elem in self.table[x].iter() {
            match elem {
                Some(value) => {
                    if value.data.key == key {
                        return Ok(**(value.data.data.as_ref()).unwrap());
                    }
                },
                _ => {},
            }
        }

        return Err(HashTableError::NotFound);
    }
}

//impl<T, U, const N: T> HashFn<T> for ChainedHash<T, U, N> where T: std::ops::Rem<usize, Output = usize> {
impl<U: std::cmp::PartialEq> HashFn for ChainedHash<U> {
    fn hash(&self, key: u16) -> u16 {
        key % self.get_capacity()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_basic_hash() {
        let mut x = ChainedHashBuilder::<u16>::new().with_capacity(3).build();
        assert_eq!(x.get_capacity(), 3);
        let mut item = 10;
        assert!(x.insert(1, item).is_ok());
        item += 10;
        assert!(x.insert(2, item).is_ok());
        item += 10;
        assert!(x.insert(3, item).is_ok());
        item += 10;
        assert_eq!(x.insert(4, item).is_ok(), true);
    }

    #[test]
    fn can_create_basic_hash_and_search() {
        let mut x = ChainedHashBuilder::<u16>::new().with_capacity(3).build();
        assert_eq!(x.get_capacity(), 3);
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
        let ret = x.lookup(17);
        assert_eq!(ret.is_ok(), false);
    }

        #[test]
        fn can_create_basic_hash_and_delete() {
            let mut x = ChainedHashBuilder::<u16>::new().with_capacity(3).build();
            assert_eq!(x.get_capacity(), 3);
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
