use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};
use std::io::Error;

use crate::{
    instructions::{deposit, initialize, withdraw},
    state::SolAccountInstruction,
};

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: Result<SolAccountInstruction, Error> =
        SolAccountInstruction::try_from_slice(instruction_data);

    match instruction {
        Ok(SolAccountInstruction::Initialize()) => initialize(),

        Ok(SolAccountInstruction::DepositSol(amount)) => deposit(amount),

        Ok(SolAccountInstruction::WithdrawSol()) => withdraw(),

        Err(err) => {
            msg!("An error occured: {}", err);

            Err(ProgramError::InvalidInstructionData)
        }
    }
}
