use std::fmt;
use std::ops::Add;
use rust_decimal::Decimal;
use crate::ledger::{Ledger, Ledgers};

#[derive(Debug, PartialEq)]
pub enum BalanceError {
    InvalidCurrency,
    BalanceNotEnough,
}

#[derive(Debug, PartialEq)]
pub struct Balance {
    currency: String,
    ledgers: Ledgers,
}

impl Balance {
    pub fn new(currency: &str) -> Result<Balance, BalanceError> {
        if currency.is_empty() {
            return Err(BalanceError::InvalidCurrency);
        }
        let currency = currency.to_string().to_uppercase();
        return Ok(Balance { currency, ledgers: Ledgers::new() });
    }

    pub fn mutate(&mut self, ledger: Ledger) -> Result<Decimal, BalanceError> {
        let amount = ledger.amount();

        let current_balance = self.ledgers.sum();
        if current_balance.add(amount).is_sign_negative() {
            return Err(BalanceError::BalanceNotEnough);
        }

        let _ = self.ledgers.add(ledger);

        let total = self.ledgers.sum();
        return Ok(total);
    }

    pub fn amount(&self) -> Decimal {
        if self.ledgers.len() == 0 {
            return Decimal::default();
        }
        self.ledgers.sum()
    }
}

impl fmt::Display for Balance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.amount().is_integer() {
            return write!(f, "{} {:.0}", self.currency, self.amount());
        }
        write!(f, "{} {}", self.currency, self.amount().to_string())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    use crate::ledger::Action;

    #[test]
    fn test_balance_new_valid_currency() {
        let balance = Balance::new("USD");
        assert!(balance.is_ok());
        let balance = balance.unwrap();
        assert_eq!(balance.currency, "USD");
    }

    #[test]
    fn test_balance_new_empty_currency() {
        let balance = Balance::new("");
        assert!(matches!(balance, Err(BalanceError::InvalidCurrency)));
    }

    #[test]
    fn test_balance_mutate_with_sufficient_funds() {
        let mut balance = Balance::new("USD").unwrap();
        let ledger = Ledger::new(Action::Deposit("100.0".to_string())).unwrap();
        let result = balance.mutate(ledger);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), dec!(100.0));
    }

    #[test]
    fn test_balance_mutate_with_insufficient_funds() {
        let mut balance = Balance::new("USD").unwrap();
        let ledger = Ledger::new(Action::Withdrawal("100.0".to_string())).unwrap();
        let result = balance.mutate(ledger);
        assert!(matches!(result, Err(BalanceError::BalanceNotEnough)));
    }

    #[test]
    fn test_balance_amount_with_no_ledgers() {
        let balance = Balance::new("USD").unwrap();
        assert_eq!(balance.amount(), Decimal::default());
    }

    #[test]
    fn test_balance_amount_with_ledgers() {
        let mut balance = Balance::new("USD").unwrap();
        let ledger_deposit = Ledger::new(Action::Deposit("100.0".to_string())).unwrap();
        balance.mutate(ledger_deposit).unwrap();
        let ledger_withdrawal = Ledger::new(Action::Withdrawal("50.0".to_string())).unwrap();
        balance.mutate(ledger_withdrawal).unwrap();
        assert_eq!(balance.amount(), dec!(50.0));
    }
}
