use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::Instruction,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::{create_account, transfer},
    sysvar::Sysvar,
};
use std::slice::Iter;

use crate::state::PDA_SOL_ACCOUNT_SEED;

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

    // Derive PDA
    let (pda, bump_seed) = Pubkey::find_program_address(
        &[PDA_SOL_ACCOUNT_SEED, user_account.key.as_ref()],
        program_id,
    );

    if pda != *sol_account.key {
        return Err(ProgramError::InvalidArgument);
    }

    // Check if PDA exists and create it if it doesn't
    if sol_account.owner == system_program.key {
        let rent: u64 = Rent::get()?.minimum_balance(0);

        let create_pda_instruction: Instruction =
            create_account(user_account.key, sol_account.key, rent, 0, program_id);

        invoke_signed(
            &create_pda_instruction,
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
    }

    // Transfer lamports from user's account to PDA
    let transfer_instruction: Instruction = transfer(user_account.key, sol_account.key, amount);

    invoke_signed(
        &transfer_instruction,
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

    Ok(())
}
