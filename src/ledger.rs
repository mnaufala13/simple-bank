use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Add;
use std::rc::Rc;
use rand::{distributions::Alphanumeric, thread_rng};
use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;

#[derive(Debug, PartialEq)]
pub enum LedgerError {
    EmptyAmount,
    ParseAmount,
    InvalidAmount(String),
    DuplicateLedger, // Add this line
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Withdrawal(String),
    Deposit(String),
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Action::Deposit(_) => write!(f, "Deposit"),
            Action::Withdrawal(_) => write!(f, "Withdrawal"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Ledger {
    id: Rc<String>,
    action: String,
    amount: Decimal,
}

impl fmt::Display for Ledger {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:?} {}", self.id, self.action, self.amount)
    }
}

impl Ledger {
    pub fn new(action: Action) -> Result<Ledger, LedgerError> {
        let amount = match &action {
            Action::Withdrawal(a) | Action::Deposit(a) if a.is_empty() => {
                return Err(LedgerError::EmptyAmount);
            }
            Action::Withdrawal(a) | Action::Deposit(a) if a.starts_with("-") => {
                let msg = "amount can't be negative".to_string();
                return Err(LedgerError::InvalidAmount(msg));
            }
            Action::Withdrawal(a) | Action::Deposit(a) => {
                a.parse::<f64>()
            }
        }.map_err(|_| LedgerError::ParseAmount)?;

        if amount == 0.0 {
            let msg = "amount can't zero".to_string();
            return Err(LedgerError::InvalidAmount(msg));
        }

        let amount = match &action {
            Action::Deposit(_) => amount,
            Action::Withdrawal(_) => amount * -1.0,
        };

        Ok(Ledger {
            id: Rc::new(generate_random_string(16)),
            action: action.to_string(),
            amount: Decimal::from_f64(amount).unwrap(),
        })
    }
    pub fn amount(&self) -> Decimal {
        self.amount.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct Ledgers {
    index: HashSet<Rc<String>>,
    pub collection: Vec<Ledger>,
}

impl Ledgers {
    pub fn new() -> Ledgers {
        Ledgers { index: HashSet::new(), collection: vec![] }
    }
    pub fn add(&mut self, ledger: Ledger) -> Result<(), LedgerError> {
        let id = ledger.id.clone();
        if self.index.contains(&id) {
            return Err(LedgerError::DuplicateLedger);
        }
        self.index.insert(id);
        self.collection.push(ledger);
        Ok(())
    }
    pub fn len(&self) -> usize {
        self.collection.len()
    }
    pub fn sum(&self) -> Decimal {
        let mut total = Decimal::default();
        for l in &self.collection {
            total = total.add(l.amount());
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
    
    use rust_decimal_macros::dec;

    #[test]
    fn test_ledger_new_withdrawal_positive_amount() {
        let action = Action::Withdrawal("100.0".to_string());
        let ledger = Ledger::new(action).unwrap();
        assert_eq!(ledger.amount(), dec!(-100.0));
    }

    #[test]
    fn test_ledger_new_deposit_positive_amount() {
        let action = Action::Deposit("200.0".to_string());
        let ledger = Ledger::new(action).unwrap();
        assert_eq!(ledger.amount(), dec!(200.0));
    }

    #[test]
    fn test_ledger_new_withdrawal_empty_amount() {
        let action = Action::Withdrawal("".to_string());
        let result = Ledger::new(action);
        assert!(matches!(result, Err(LedgerError::EmptyAmount)));
    }

    #[test]
    fn test_ledger_new_deposit_empty_amount() {
        let action = Action::Deposit("".to_string());
        let result = Ledger::new(action);
        assert!(matches!(result, Err(LedgerError::EmptyAmount)));
    }

    #[test]
    fn test_ledger_new_withdrawal_negative_amount() {
        let action = Action::Withdrawal("-50.0".to_string());
        let result = Ledger::new(action);
        assert!(matches!(result, Err(LedgerError::InvalidAmount(_))));
    }

    #[test]
    fn test_ledger_new_deposit_negative_amount() {
        let action = Action::Deposit("-50.0".to_string());
        let result = Ledger::new(action);
        assert!(matches!(result, Err(LedgerError::InvalidAmount(_))));
    }

    #[test]
    fn test_ledger_new_zero_amount() {
        let action = Action::Deposit("0.0".to_string());
        let result = Ledger::new(action);
        assert!(matches!(result, Err(LedgerError::InvalidAmount(_))));
    }

    #[test]
    fn test_ledgers_new() {
        let ledgers = Ledgers::new();
        assert_eq!(ledgers.len(), 0);
    }

    #[test]
    fn test_ledgers_add() {
        let mut ledgers = Ledgers::new();
        let action = Action::Deposit("100.0".to_string());
        let ledger = Ledger::new(action).unwrap();
        let _ = ledgers.add(ledger);
        assert_eq!(ledgers.len(), 1);
    }

    #[test]
    fn test_ledgers_sum() {
        let mut ledgers = Ledgers::new();
        let action_deposit = Action::Deposit("100.0".to_string());
        let ledger_deposit = Ledger::new(action_deposit).unwrap();
        let _ = ledgers.add(ledger_deposit);

        let action_withdrawal = Action::Withdrawal("50.0".to_string());
        let ledger_withdrawal = Ledger::new(action_withdrawal).unwrap();
        let _ = ledgers.add(ledger_withdrawal);

        assert_eq!(ledgers.sum(), rust_decimal_macros::dec!(50.0));
    }

    #[test]
    fn test_ledgers_add_duplicate() {
        let mut ledgers = Ledgers::new();
        let action1 = Action::Deposit("100.0".to_string());
        let action2 = Action::Deposit("100.0".to_string());
        let ledger1 = Ledger::new(action1).unwrap();
        let mut ledger2 = Ledger::new(action2).unwrap();
        ledger2.id = ledger1.id.clone();

        assert_eq!(ledgers.add(ledger1), Ok(()));
        assert_eq!(ledgers.add(ledger2), Err(LedgerError::DuplicateLedger));
    }
}