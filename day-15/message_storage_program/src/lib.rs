use core::str;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct MessageAccount {
    pub message: String,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let new_message = str::from_utf8(instruction_data).map_err(|_| solana_program::program_error::ProgramError::InvalidInstructionData)?;

    let mut message_account = MessageAccount::try_from_slice(&account.data.borrow())?;

    message_account.message = new_message.to_string();

    message_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
