#[derive(Debug)]
pub enum HashTableError {
    NotFound,
    TableFull,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashTableEntry<U> {
    pub key: u16,
    pub data: Option<Box<U>>,
}

impl<U: Default> Default for HashTableEntry<U> {
    fn default() -> HashTableEntry<U> {
        HashTableEntry::<U> {
            key: u16::default(),
            data: None,
        }
    }
}

pub trait HashTable<U> where U: Copy {
    fn insert(&mut self, key: u16, data: U) -> Result<(), HashTableError>;
    fn delete(&mut self, key: u16) -> Result<(), HashTableError>;
    fn lookup(&self, key: u16) -> Result<U, HashTableError>;
}

pub trait HashFn {
    fn hash(&self, key: u16) -> u16;
}

