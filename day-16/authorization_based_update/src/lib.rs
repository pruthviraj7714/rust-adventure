use std::borrow::Borrow;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AdminAccount {
    pub admin_pubkey: Pubkey,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TargetAccount {
    pub user_pubkey: Pubkey,
    pub role: u8,
    pub status: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum CustomError {
    UnauthorizedAccess,
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

// Entry point for the program
entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let admin_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;
    let target_account = next_account_info(accounts_iter)?;

    handle_authorize_update(
        program_id,
        admin_account,
        user_account,
        target_account,
        instruction_data,
    )
}

pub fn handle_authorize_update(
    program_id: &Pubkey,
    admin_account: &AccountInfo,
    _user_account: &AccountInfo,
    target_account: &AccountInfo,
    instruction_data: &[u8],
) -> ProgramResult {
    // Deserialize the admin account to get the admin pubkey
    let admin_data = AdminAccount::try_from_slice(&admin_account.data.borrow())?;

    if !admin_account.is_signer || admin_data.admin_pubkey != *admin_account.key {
        msg!("Unauthorized access: Admin account required");
        return Err(CustomError::UnauthorizedAccess.into());
    }

    // Deserialize the target account to get current role/status
    let mut target_data = TargetAccount::try_from_slice(&target_account.data.borrow())?;

    // Read new role/status values from instruction data
    let new_role = instruction_data[0];
    let new_status = instruction_data[1];

    // Update target account fields
    target_data.role = new_role;
    target_data.status = new_status;

    // Serialize and save updated target data back to account
    target_data.serialize(&mut &mut target_account.data.borrow_mut()[..])?;

    msg!("Role and status updated successfully");

    Ok(())
}
