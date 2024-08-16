use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction::transfer,
};
use std::{cell::RefMut, slice::Iter};

use crate::state::{SolAccount, PDA_SOL_ACCOUNT_SEED};

pub fn withdraw(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter: &mut Iter<AccountInfo> = &mut accounts.iter();

    let user_account: &AccountInfo = next_account_info(accounts_iter)?;
    let sol_account: &AccountInfo = next_account_info(accounts_iter)?;
    let system_program: &AccountInfo = next_account_info(accounts_iter)?;

    // Ensure the user is a signer
    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[PDA_SOL_ACCOUNT_SEED, user_account.key.as_ref()],
        program_id,
    );

    if sol_account.key != &pda {
        return Err(ProgramError::InvalidArgument);
    }

    // Ensure PDA exists
    if sol_account.data_is_empty() {
        return Err(ProgramError::UninitializedAccount);
    }

    // Borrow and unpack PDA account data
    let sol_account_data: &mut RefMut<&mut [u8]> = &mut sol_account.try_borrow_mut_data()?;
    let mut sol_data: SolAccount = SolAccount::unpack(sol_account_data)?;

    if sol_data.owner != *user_account.key {
        return Err(ProgramError::IllegalOwner);
    }

    let amount: u64 = sol_data
        .balance
        .checked_div(10)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    if amount > sol_account.lamports() {
        return Err(ProgramError::InsufficientFunds);
    }

    // Transfer lamports from PDA to user
    let transfer_ix: Instruction = transfer(sol_account.key, user_account.key, amount);

    invoke_signed(
        &transfer_ix,
        &[
            sol_account.clone(),
            user_account.clone(),
            system_program.clone(),
        ],
        &[&[
            PDA_SOL_ACCOUNT_SEED,
            user_account.key.as_ref(),
            &[bump_seed],
        ]],
    )?;

    // Update PDA data
    sol_data.balance = sol_data
        .balance
        .checked_sub(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    SolAccount::pack(sol_data, sol_account_data)?;

    Ok(())
}
