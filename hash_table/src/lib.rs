#[derive(Debug)]
pub enum HashTableError {
    NotFound,
    TableFull,
}

#[derive(Debug)]
pub struct HashTableEntry<U> {
    pub valid: bool,
    pub key: u16,
    pub data: Box<U>,
}

impl<U: Default> Default for HashTableEntry<U> {
    fn default() -> HashTableEntry<U> {
        HashTableEntry::<U> {
            valid: false,
            key: u16::default(),
            data: Box::new(U::default()),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
