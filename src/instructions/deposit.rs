use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use std::{cell::RefMut, slice::Iter};

use crate::state::SolAccount;

pub fn deposit(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter: &mut Iter<AccountInfo> = &mut accounts.iter();

    let user_account: &AccountInfo = next_account_info(accounts_iter)?;
    let sol_account: &AccountInfo = next_account_info(accounts_iter)?;
    let _system_program: &AccountInfo = next_account_info(accounts_iter)?;

    if sol_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    let sol_account_data: &mut RefMut<&mut [u8]> = &mut sol_account.try_borrow_mut_data()?;

    let mut sol_data: SolAccount = SolAccount::unpack(sol_account_data)?;

    if !sol_data.is_initialized {
        let rent: Rent = Rent::get()?;
        let required_lamports: u64 = rent.minimum_balance(SolAccount::LEN);

        if sol_account.lamports() < required_lamports {
            return Err(ProgramError::InsufficientFunds);
        }

        sol_data.is_initialized = true;
        sol_data.owner = *user_account.key;
        sol_data.balance = 0;

        msg!("Account initialized!");
    }

    sol_data.balance = sol_data
        .balance
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    SolAccount::pack(sol_data, sol_account_data)?;

    **sol_account.try_borrow_mut_lamports()? = sol_account
        .lamports()
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    **user_account.try_borrow_mut_lamports()? = user_account
        .lamports()
        .checked_sub(amount)
        .ok_or(ProgramError::InsufficientFunds)?;

    Ok(())
}
