#[derive(Debug)]
pub enum HashTableError {
    NotFound,
    TableFull,
}

#[derive(Debug)]
pub struct HashTableEntry<T> {
    pub valid: bool,
    pub data: Box<T>,
}

impl<T: Default> Default for HashTableEntry<T> {
    fn default() -> HashTableEntry<T> {
        HashTableEntry::<T> {
            valid: false,
            data: Box::new(T::default()),
        }
    }
}

pub trait HashTable<T, U> {
    fn insert(&mut self, key: T, data: HashTableEntry<U>) -> Result<(), HashTableError>;
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
