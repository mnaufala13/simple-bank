mod balance;
mod ledger;
mod storage;

use balance::Balance;
use ledger::{Action, Ledger};
use storage::InMemory;

fn main() {
    let mut storage = InMemory::new();
    let account_id = "account_1";

    // Create a new balance for USD currency
    match Balance::new("USD") {
        Ok(balance) => {
            if let Err(e) = storage.insert(account_id, balance) {
                println!("Error inserting balance: {:?}", e);
                return;
            }
        }
        Err(e) => {
            println!("Error creating balance: {:?}", e);
            return;
        }
    }

    // Simulate deposit
    match simulate_deposit(&mut storage, account_id, 100.0) {
        Ok(_) => println!("Deposit successful"),
        Err(e) => println!("Error during deposit: {:?}", e),
    }

    // Simulate withdrawal
    match simulate_withdrawal(&mut storage, account_id, 50.0) {
        Ok(_) => println!("Withdrawal successful"),
        Err(e) => println!("Error during withdrawal: {:?}", e),
    }

    // Simulate deposit
    match simulate_deposit(&mut storage, account_id, 100.0) {
        Ok(_) => println!("Deposit successful"),
        Err(e) => println!("Error during deposit: {:?}", e),
    }

    // Display final balance
    if let Some(balance) = storage.get_mut(account_id) {
        println!("Final balance: {}", balance);
    } else {
        println!("Account not found");
    }
}

fn simulate_deposit(storage: &mut InMemory, account_id: &str, amount: f64) -> Result<(), String> {
    let action = Action::Deposit(amount.to_string());
    let ledger = Ledger::new(action).map_err(|e| format!("Error creating ledger: {:?}", e))?;
    if let Some(balance) = storage.get_mut(account_id) {
        balance
            .mutate(ledger)
            .map_err(|e| format!("Error mutating balance: {:?}", e))?;
    } else {
        return Err("Account not found".to_string());
    }
    Ok(())
}

fn simulate_withdrawal(
    storage: &mut InMemory,
    account_id: &str,
    amount: f64,
) -> Result<(), String> {
    let action = Action::Withdrawal(amount.to_string());
    let ledger = Ledger::new(action).map_err(|e| format!("Error creating ledger: {:?}", e))?;
    if let Some(balance) = storage.get_mut(account_id) {
        balance
            .mutate(ledger)
            .map_err(|e| format!("Error mutating balance: {:?}", e))?;
    } else {
        return Err("Account not found".to_string());
    }
    Ok(())
}
