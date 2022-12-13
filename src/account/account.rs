use crate::errors::account::{BalanceError, DepositionError, WithdrawalError};

#[derive(Copy, Clone, Debug)]
pub struct CurrentAccount(pub u128);
#[derive(Copy, Clone, Debug)]
pub struct SavingsAccount(pub u128);
#[derive(Copy, Clone, Debug)]
pub struct FrozenAccount(pub u128);
#[derive(Copy, Clone, Debug)]
pub struct DynamicAccount(pub AccountState);


#[derive(Copy, Clone, Debug)]
pub enum AccountState {
    CURRENT(CurrentAccount),
    SAVINGS(SavingsAccount),
    FROZEN(FrozenAccount),
}


#[allow(unused_variables)]
pub trait Account {
    fn balance(&self) -> Result<f64, BalanceError> {
        Err(BalanceError("Cannot check balance of this account"))
    }

    fn deposit(&mut self, amount: f64) -> Result<f64, DepositionError> {
        Err(DepositionError("Cannot desposit money into this account"))
    }

    fn withdraw(&mut self, amount: f64) -> Result<f64, WithdrawalError> {
        Err(WithdrawalError("Cannot withdraw money from this account"))
    }
}


impl Account for CurrentAccount {
    fn balance(&self) -> Result<f64, BalanceError> {
        Ok((self.0 / 100) as f64)
    }

    fn deposit(&mut self, amount: f64) -> Result<f64, DepositionError> {
        self.0 += (amount * 100.0).floor() as u128;

        Ok(self.balance().unwrap())
    }

    fn withdraw(&mut self, amount: f64) -> Result<f64, WithdrawalError> {
        if self.0 < (amount * 100.0).floor() as u128 {
            return Err(WithdrawalError("Insufficient funds in account"));
        }

        self.0 -= (amount * 100.0).floor() as u128;

        Ok(self.balance().unwrap())
    }
}
