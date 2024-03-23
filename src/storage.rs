use std::collections::HashMap;

use crate::balance::Balance;

#[derive(Debug)]
pub enum StorageError {
    AccountAlreadyExists,
}

pub struct InMemory {
    balances: HashMap<String, Balance>,
}

impl InMemory {
    pub fn new() -> Self {
        Self {
            balances: HashMap::new(),
        }
    }

    pub fn get_mut(&mut self, account_id: &str) -> Option<&mut Balance> {
        self.balances.get_mut(account_id)
    }

    pub fn insert(&mut self, account_id: &str, balance: Balance) -> Result<(), StorageError> {
        if self.balances.contains_key(account_id) {
            return Err(StorageError::AccountAlreadyExists);
        }
        self.balances.insert(account_id.to_string(), balance);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::balance::Balance;

    #[test]
    fn test_new_in_memory() {
        let storage = InMemory::new();
        assert!(storage.balances.is_empty());
    }

    #[test]
    fn test_insert_and_get_mut_balance() {
        let mut storage = InMemory::new();
        let balance = Balance::new("USD").unwrap();
        assert!(storage.insert("account_1", balance).is_ok());

        if let Some(balance) = storage.get_mut("account_1") {
            assert_eq!(balance.currency, "USD");
        } else {
            panic!("Balance for account_1 should exist");
        }
    }

    #[test]
    fn test_insert_existing_account() {
        let mut storage = InMemory::new();
        let balance = Balance::new("USD").unwrap();
        assert!(storage.insert("account_1", balance.clone()).is_ok());
        assert!(matches!(
            storage.insert("account_1", balance),
            Err(StorageError::AccountAlreadyExists)
        ));
    }
}
