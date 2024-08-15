use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub fn deposit(_program_id: &Pubkey, _accounts: &[AccountInfo], _amount: u64) -> ProgramResult {
    Ok(())
}
