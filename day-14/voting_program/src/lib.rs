use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::ProgramError,
};

// Define the voting account structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterAccount {
    counter : u32
}

// Program entrypoint
entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    counter_account.counter += 1;

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
