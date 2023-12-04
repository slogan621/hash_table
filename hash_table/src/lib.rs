#[derive(Debug)]
pub enum HashTableError {
    NotFound,
    TableFull,
}

#[derive(Debug, Default)]
pub struct HashTableEntry {
    pub valid: bool,
}

pub trait HashTable<T> {
    fn insert(&mut self, key: T, data: HashTableEntry) -> Result<(), HashTableError>;
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
