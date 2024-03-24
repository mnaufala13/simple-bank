use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::balance::Balance;

#[derive(Debug)]
pub enum StorageError {
    AccountAlreadyExists,
    AccountNotExists
}

pub struct InMemory {
    balances: Arc<RwLock<HashMap<String, Balance>>>,
}

impl Clone for InMemory {
    fn clone(&self) -> Self {
        InMemory {
            balances: self.balances.clone(),
        }
    }
}

impl InMemory {
    pub fn new() -> Self {
        Self {
            balances: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn get(&self, account_id: &str) -> Option<Balance> {
        let bal = self.balances.read().unwrap();
        let x = bal.get(account_id);
        match x {
            Some(b) => Some(b.clone()),
            None => None,
        }
    }

    pub fn insert(&mut self, account_id: &str, balance: Balance) -> Result<(), StorageError> {
        let mut bal = self.balances.write().unwrap();
        if bal.contains_key(account_id) {
            return Err(StorageError::AccountAlreadyExists);
        }
        bal.insert(account_id.to_string(), balance);
        Ok(())
    }

    pub fn update(&mut self, account_id: &str, balance: Balance) -> Result<(), StorageError> {
        let mut bal = self.balances.write().unwrap();
        if !bal.contains_key(account_id) {
            return Err(StorageError::AccountNotExists);
        }
        bal.insert(account_id.to_string(), balance);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;
    use rust_decimal::prelude::FromPrimitive;
    use super::*;
    use crate::ledger::{Action, Ledger};

    #[test]
    fn test_get_balance() {
        let mut storage = InMemory::new();
        let balance = Balance::new("USD").unwrap();
        storage.insert("account_1", balance.clone()).unwrap();
        let balance = storage.get("account_1").unwrap();
        assert_eq!(balance.currency, "USD");
    }

    #[test]
    fn test_new_in_memory() {
        let storage = InMemory::new();
        assert!(storage.balances.read().unwrap().is_empty());
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

    #[test]
    fn test_insert_multiple_account_using_multiple_threads() {
        let storage = InMemory::new();

        let balance = Balance::new("USD").unwrap();
        let balance_clone = balance.clone();
        let mut storage_clone = storage.clone();
        let handle = std::thread::spawn(move || {
            storage_clone.insert("account_1", balance_clone).unwrap();
        });

        let balance_clone = balance.clone();
        let mut storage_clone = storage.clone();
        let handle2 = std::thread::spawn(move || {
            storage_clone.insert("account_2", balance_clone).unwrap();
        });

        handle.join().unwrap();
        handle2.join().unwrap();

        assert_eq!(storage.get("account_1").unwrap().currency, "USD");
        assert_eq!(storage.get("account_2").unwrap().currency, "USD");
    }

    #[test]
    fn test_update_balance_with_exist_account() {
        let mut storage = InMemory::new();
        let balance = Balance::new("USD").unwrap();
        storage.insert("account_1", balance.clone()).unwrap();

        let mut balance = storage.get("account_1").unwrap();
        let ledger = Ledger::new(Action::Deposit("100.0".to_string())).unwrap();
        balance.mutate(ledger).unwrap();

        storage.update("account_1", balance).unwrap();

        let balance = storage.get("account_1").unwrap();
        assert_eq!(balance.amount(), Decimal::from_f64(100.0).unwrap());
    }

    #[test]
    fn test_update_balance_with_not_exist_account() {
        let mut storage = InMemory::new();
        let balance = Balance::new("USD").unwrap();
        assert!(matches!(
            storage.update("account_1", balance),
            Err(StorageError::AccountNotExists)
        ));
    }
}
