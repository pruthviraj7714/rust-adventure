use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CountDownAccount {
    count: i32,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CountDownAccount::try_from_slice(&account.data.borrow())?;

    // Check if count is zero or below
    if counter_account.count <= 0 {
        return Err(ProgramError::Custom(0)); // Use a custom error code for countdown reached zero
    }

    counter_account.count -= 1;

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
