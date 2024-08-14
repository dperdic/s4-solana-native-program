use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SolAccountInstruction {
    Initialize(),
    DepositSol(u64),
    WithdrawSol(),
}
