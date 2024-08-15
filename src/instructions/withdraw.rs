use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
};
use std::{cell::RefMut, slice::Iter};

use crate::state::SolAccount;

pub fn withdraw(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter: &mut Iter<AccountInfo> = &mut accounts.iter();

    let user_account: &AccountInfo = next_account_info(accounts_iter)?;
    let sol_account: &AccountInfo = next_account_info(accounts_iter)?;
    let _system_program: &AccountInfo = next_account_info(accounts_iter)?;

    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let sol_account_data: &mut RefMut<&mut [u8]> = &mut sol_account.try_borrow_mut_data()?;

    let mut sol_data: SolAccount = SolAccount::unpack(sol_account_data)?;

    if !sol_data.is_initialized {
        return Err(ProgramError::UninitializedAccount);
    }

    if sol_data.owner != *user_account.key {
        return Err(ProgramError::IllegalOwner);
    }

    let withdraw_amount: u64 = sol_data
        .balance
        .checked_div(10)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    if withdraw_amount > sol_account.lamports() {
        return Err(ProgramError::InsufficientFunds);
    }

    sol_data.balance = sol_data
        .balance
        .checked_sub(withdraw_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    SolAccount::pack(sol_data, sol_account_data)?;

    **sol_account.try_borrow_mut_lamports()? = sol_account
        .lamports()
        .checked_sub(withdraw_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    **user_account.try_borrow_mut_lamports()? = user_account
        .lamports()
        .checked_add(withdraw_amount)
        .ok_or(ProgramError::InsufficientFunds)?;

    Ok(())
}
