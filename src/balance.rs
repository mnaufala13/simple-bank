use std::{fmt, str::FromStr};
use std::ops::Add;
use rust_decimal::{Decimal};
use crate::ledger::{Ledger, Ledgers, Action};

#[derive(Debug, PartialEq)]
pub enum BalanceError {
    ParseAmount(rust_decimal::Error),
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
        let amount = Decimal::from_str(ledger.amount().as_str()).
            map_err(|e| BalanceError::ParseAmount(e))?;

        let current_balance = self.ledgers.sum();
        if current_balance.add(amount).is_sign_negative() {
            return Err(BalanceError::BalanceNotEnough);
        }

        self.ledgers.add(ledger);

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
mod test {
    use super::*;

    #[test]
    fn test_balance_new_empty_currency() {
        let e = Balance::new("").unwrap_err();
        assert_eq!(e, BalanceError::InvalidCurrency)
    }

    #[test]
    fn test_balance_new_valid_currency() {
        let b = Balance::new("USD").unwrap();
        assert_eq!(b.currency, "USD")
    }

    #[test]
    fn test_balance_mutate_negative_balance() {
        let mut b = Balance::new("USD").unwrap();
        let ledger = Ledger::new(Action::Withdrawal, "-200.00").unwrap();
        let e = b.mutate(ledger).unwrap_err();
        assert_eq!(e, BalanceError::BalanceNotEnough)
    }

    #[test]
    fn test_balance_mutate_valid_amount() {
        let mut b = Balance::new("USD").unwrap();
        let ledger = Ledger::new(Action::Deposit, "100.00").unwrap();
        let total = b.mutate(ledger).unwrap();
        assert_eq!(total, Decimal::new(100, 0))
    }
}