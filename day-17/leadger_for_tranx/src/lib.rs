use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Transaction {
    sender: Pubkey,
    reciever: Pubkey,
    amount: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LedgerAccount {
    transactions: Vec<Transaction>,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut ledger_account = LedgerAccount::try_from_slice(&account.data.borrow())?;

    let new_transaction = Transaction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    if ledger_account.transactions.len() >= 10 {
        ledger_account.transactions.remove(0);
    }

    ledger_account.transactions.push(new_transaction);

    ledger_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
