use core::str;
use std::collections::HashMap;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, 
    entrypoint, 
    entrypoint::ProgramResult, 
    program_error::ProgramError, 
    pubkey::Pubkey
};
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UsersDataAccount {
    users: HashMap<String, i32>,
}

entrypoint!(process_instrution);

fn process_instrution(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut users_data = UsersDataAccount::try_from_slice(&account.data.borrow())?;

    let user_id = str::from_utf8(&instruction_data[0..4])
        .map_err(|_| ProgramError::InvalidInstructionData)?
        .to_string();

    let amount = i32::from_be_bytes(instruction_data[4..8].try_into().unwrap());

    if let Some(balance) = users_data.users.get_mut(&user_id) {
        if *balance + amount < 0 {
            return Err(ProgramError::InsufficientFunds);
        }
        *balance -= amount;
    } else {
        return Err(ProgramError::InvalidAccountData);
    }

    users_data.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
