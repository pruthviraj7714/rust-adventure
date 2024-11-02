use core::str;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct ListAccount {
    str_list: Vec<String>,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // Deserialize account data
    let mut list_account = ListAccount::try_from_slice(&account.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let string_data = str::from_utf8(instruction_data).map_err(|_| ProgramError::InvalidInstructionData)?;

    let new_strings: Vec<String> = string_data.split_whitespace().map(String::from).collect();

    for new_string in new_strings {
        if list_account.str_list.len() < 5 {
            list_account.str_list.push(new_string);
        } else {
            break; 
        }
    }

    // Serialize and store the updated account data
    list_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
