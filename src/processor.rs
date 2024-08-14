use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum TransferInstruction {
    Initialize(),
    DepositSol(u64),
    WithdrawSol(),
}

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = TransferInstruction::try_from_slice(instruction_data);

    match instruction {
        Ok(TransferInstruction::Initialize()) => Ok(()),

        Ok(TransferInstruction::DepositSol(args)) => Ok(()),

        Ok(TransferInstruction::WithdrawSol()) => Ok(()),

        Err(err) => Err(ProgramError::InvalidInstructionData),
    }
}
