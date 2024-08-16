use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::{
    instructions::{deposit, withdraw},
    state::SolAccountInstruction,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: SolAccountInstruction =
        SolAccountInstruction::try_from_slice(instruction_data)?;

    match instruction {
        SolAccountInstruction::DepositSol(amount) => deposit(program_id, accounts, amount),
        SolAccountInstruction::WithdrawSol => withdraw(program_id, accounts),
    }
}
