pub mod account {
    #[derive(Copy, Clone, Debug)]
    pub struct BalanceError(pub &'static str);
    #[derive(Copy, Clone, Debug)]
    pub struct DepositionError(pub &'static str);
    #[derive(Copy, Clone, Debug)]
    pub struct WithdrawalError(pub &'static str);
}
