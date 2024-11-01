use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    positive_integer_count: u32,
    negative_integer_count: u32,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())
        .map_err(|_| ProgramError::InvalidAccountData)?;

    let number = i8::from_le_bytes([instruction_data[0]]);

    if number > 0 {
        counter_account.positive_integer_count += 1;
    } else if number < 0 {
        counter_account.negative_integer_count += 1;
    } 

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
