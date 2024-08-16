use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::slice::Iter;

use crate::state::PDA_SOL_ACCOUNT_SEED;

pub fn withdraw(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter: &mut Iter<AccountInfo> = &mut accounts.iter();

    let user_account: &AccountInfo = next_account_info(accounts_iter)?;
    let sol_account: &AccountInfo = next_account_info(accounts_iter)?;
    let _system_program: &AccountInfo = next_account_info(accounts_iter)?;

    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[PDA_SOL_ACCOUNT_SEED, user_account.key.as_ref()],
        program_id,
    );

    if pda != *sol_account.key {
        return Err(ProgramError::InvalidArgument);
    }

    if sol_account.owner != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    let sol_account_balance = sol_account.lamports();

    let amount = sol_account_balance
        .checked_div(10)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    **sol_account.try_borrow_mut_lamports()? -= amount;
    **user_account.try_borrow_mut_lamports()? += amount;

    Ok(())
}
