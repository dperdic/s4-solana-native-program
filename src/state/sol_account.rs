use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use std::mem::size_of;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SolAccountInstruction {
    DepositSol(u64),
    WithdrawSol(),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SolAccount {
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub balance: u64,
}

impl Sealed for SolAccount {}

impl IsInitialized for SolAccount {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for SolAccount {
    const LEN: usize = size_of::<SolAccount>();

    fn unpack_from_slice(src: &[u8]) -> Result<Self, solana_program::program_error::ProgramError> {
        let is_initialized: bool = src[0] != 0;

        let owner_bytes: [u8; 32] =
            <[u8; 32]>::try_from(&src[1..33]).map_err(|_| ProgramError::InvalidAccountData)?;

        let balance_bytes: [u8; 8] =
            <[u8; 8]>::try_from(&src[33..]).map_err(|_| ProgramError::InvalidAccountData)?;

        let owner: Pubkey = Pubkey::new_from_array(owner_bytes);
        let balance: u64 = u64::from_le_bytes(balance_bytes);

        Ok(SolAccount {
            is_initialized,
            owner,
            balance,
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        dst[0] = self.is_initialized as u8;
        dst[1..33].copy_from_slice(self.owner.as_ref());
        dst[33..].copy_from_slice(&self.balance.to_be_bytes());
    }
}
