use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    sysvar::Sysvar,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Account {
    pub data: Option<String>,
    pub expires: i64,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut account_data = Account::try_from_slice(&account.data.borrow())?;

    let current_timestamp = Clock::get().unwrap().unix_timestamp;

    if account_data.expires < current_timestamp {
        account_data.data = None;
    }

    account_data.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
