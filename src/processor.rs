use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};
use std::io::Error;

use crate::{
    instructions::{deposit, withdraw},
    state::SolAccountInstruction,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: Result<SolAccountInstruction, Error> =
        SolAccountInstruction::try_from_slice(instruction_data);

    match instruction {
        Ok(SolAccountInstruction::DepositSol(amount)) => deposit(program_id, accounts, amount),

        Ok(SolAccountInstruction::WithdrawSol()) => withdraw(program_id, accounts),

        Err(err) => {
            msg!("An error occured: {}", err);

            Err(ProgramError::InvalidInstructionData)
        }
    }
}
