#[derive(Debug)]
pub enum HashTableError {
    NotFound,
    TableFull,
}

#[derive(Debug)]
pub struct HashTableEntry<T, U> {
    pub valid: bool,
    pub key: T,
    pub data: Box<U>,
}

impl<T: Default, U: Default> Default for HashTableEntry<T, U> {
    fn default() -> HashTableEntry<T, U> {
        HashTableEntry::<T, U> {
            valid: false,
            key: T::default(),
            data: Box::new(U::default()),
        }
    }
}

pub trait HashTable<T, U> {
    fn insert(&mut self, key: T, data: U) -> Result<(), HashTableError>;
    fn delete(&mut self, key: T) -> Result<(), HashTableError>;
    fn lookup(&self, key: T) -> Result<(), HashTableError>;
}

pub trait HashFn<T> {
    fn hash(&self, key: T) -> usize;
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
