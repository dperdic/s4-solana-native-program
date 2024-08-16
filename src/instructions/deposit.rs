use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::Instruction,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::{create_account, transfer},
    sysvar::Sysvar,
};
use std::{cell::RefMut, slice::Iter};

use crate::state::{SolAccount, PDA_SOL_ACCOUNT_SEED};

pub fn deposit(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let accounts_iter: &mut Iter<AccountInfo> = &mut accounts.iter();

    let user_account: &AccountInfo = next_account_info(accounts_iter)?;
    let sol_account: &AccountInfo = next_account_info(accounts_iter)?;
    let system_program: &AccountInfo = next_account_info(accounts_iter)?;

    if !user_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if user_account.lamports() < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[PDA_SOL_ACCOUNT_SEED, user_account.key.as_ref()],
        program_id,
    );

    if sol_account.key != &pda {
        return Err(ProgramError::InvalidArgument);
    }

    // Initialize PDA if it doesn't exist
    if sol_account.data_is_empty() {
        let rent: Rent = Rent::get()?;
        let required_lamports: u64 = rent.minimum_balance(SolAccount::LEN);

        let create_pda_account_ix: Instruction = create_account(
            user_account.key,
            sol_account.key,
            required_lamports,
            SolAccount::LEN as u64,
            program_id,
        );

        invoke_signed(
            &create_pda_account_ix,
            &[
                user_account.clone(),
                sol_account.clone(),
                system_program.clone(),
            ],
            &[&[
                PDA_SOL_ACCOUNT_SEED,
                user_account.key.as_ref(),
                &[bump_seed],
            ]],
        )?;

        let sol_account_data: &mut RefMut<&mut [u8]> = &mut sol_account.try_borrow_mut_data()?;

        let sol_data: SolAccount = SolAccount {
            is_initialized: true,
            owner: *user_account.key,
            balance: 0,
        };

        SolAccount::pack(sol_data, sol_account_data)?;

        msg!("SOL account created and initialized");
    }

    // Transfer lamports from user to PDA
    let transfer_ix: Instruction = transfer(user_account.key, sol_account.key, amount);

    invoke_signed(
        &transfer_ix,
        &[
            user_account.clone(),
            sol_account.clone(),
            system_program.clone(),
        ],
        &[&[
            PDA_SOL_ACCOUNT_SEED,
            user_account.key.as_ref(),
            &[bump_seed],
        ]],
    )?;

    // Borrow and unpack PDA account data
    let sol_account_data: &mut RefMut<&mut [u8]> = &mut sol_account.try_borrow_mut_data()?;
    let mut sol_data: SolAccount = SolAccount::unpack(&sol_account_data)?;

    // Update PDA balance
    sol_data.balance = sol_data
        .balance
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    SolAccount::pack(sol_data, sol_account_data)?;

    Ok(())
}
