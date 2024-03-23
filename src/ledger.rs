use std::fmt;
use std::fmt::{Formatter};
use std::ops::Add;
use std::str::FromStr;
use rand::{distributions::Alphanumeric, thread_rng};
use rand::Rng;
use rust_decimal::Decimal;

#[derive(Debug, PartialEq)]
pub enum LedgerError {
    EmptyAmount,
    ParseAmount,
    InvalidAmount(String),
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Withdrawal,
    Deposit,
}

#[derive(Debug, PartialEq)]
pub struct Ledger {
    id: String,
    action: Action,
    amount: String,
}

impl fmt::Display for Ledger {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?} {}", self.id, self.action, self.amount)
    }
}

impl Ledger {
    pub fn new(action: Action, amount: &str) -> Result<Ledger, LedgerError> {
        if amount.is_empty() {
            return Err(LedgerError::EmptyAmount);
        }
        let amount = amount.parse::<f64>().map_err(|_| LedgerError::ParseAmount)?;
        match action {
            Action::Withdrawal if amount >= 0.0 => {
                return Err(LedgerError::InvalidAmount("withdrawal amount can't positive or zero".to_string()));
            }
            Action::Deposit if amount <= 0.0 => {
                return Err(LedgerError::InvalidAmount("deposit amount can't negative or zero".to_string()));
            }
            _ => (),
        }
        Ok(Ledger {
            id: generate_random_string(16),
            action,
            amount: amount.to_string(),
        })
    }
    pub fn amount(&self) -> String {
        self.amount.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct Ledgers {
    pub collection: Vec<Ledger>,
}

impl Ledgers {
    pub fn new() -> Ledgers {
        Ledgers { collection: vec![] }
    }
    pub fn add(&mut self, ledger: Ledger) {
        self.collection.push(ledger);
    }
    pub fn len(&self) -> usize {
        self.collection.len()
    }
    pub fn sum(&self) -> Decimal {
        let mut total = Decimal::default();
        for l in &self.collection {
            let amount = Decimal::from_str(l.amount.as_str()).unwrap();
            total = total.add(amount);
        }
        total
    }
}

fn generate_random_string(len: usize) -> String {
    let s: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect::<String>();
    return s;
}

mod tests {
    use super::*;
    use super::Action::*;

    #[test]
    fn test_ledger_sum() {
        let ll = Ledgers {
            collection: vec![
                Ledger::new(Deposit, "100.5").unwrap(),
                Ledger::new(Deposit, "100.20").unwrap(),
                Ledger::new(Deposit, "10.23131").unwrap(),
                Ledger::new(Deposit, "11.0").unwrap(),
                Ledger::new(Withdrawal, "-31.22").unwrap(),
            ]
        };
        let total = ll.sum();
        assert_eq!(total.to_string(), "190.71131")
    }

    #[test]
    fn test_ledger_new() {
        let e = Ledger::new(Deposit, "100.5").unwrap();
        assert_eq!(e.action, Deposit);
        assert_eq!(e.amount, "100.5");

        let e = Ledger::new(Withdrawal, "-20.2").unwrap();
        assert_eq!(e.action, Withdrawal);
        assert_eq!(e.amount, "-20.2");
    }

    #[test]
    fn test_ledger_new_invalid_amount_withdraw() {
        let e = Ledger::new(Withdrawal, "100.5").unwrap_err();
        assert_eq!(e, LedgerError::InvalidAmount("withdrawal amount can't positive or zero".to_string()))
    }

    #[test]
    fn test_ledger_new_invalid_amount_deposit() {
        let e = Ledger::new(Deposit, "-100.5").unwrap_err();
        assert_eq!(e, LedgerError::InvalidAmount("deposit amount can't negative or zero".to_string()))
    }

    #[test]
    fn test_ledger_new_empty_amount() {
        let e = Ledger::new(Deposit, "").unwrap_err();
        assert_eq!(e, LedgerError::EmptyAmount)
    }

    #[test]
    fn test_ledger_new_parse_amount() {
        let e = Ledger::new(Deposit, "abc").unwrap_err();
        assert_eq!(e, LedgerError::ParseAmount)
    }

    #[test]
    fn test_ledgers_add() {
        let mut l = Ledgers::new();
        let ledger = Ledger::new(Deposit, "100.00").unwrap();
        l.add(ledger);
        assert_eq!(l.len(), 1);
    }
}