use borsh::{BorshDeserialize, BorshSerialize};

pub const PDA_SOL_ACCOUNT_SEED: &[u8; 11] = b"sol_account";

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SolAccountInstruction {
    DepositSol(u64),
    WithdrawSol,
}
