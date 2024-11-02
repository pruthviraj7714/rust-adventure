use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenAccount {
    balance: u32,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let sender_account = next_account_info(accounts_iter)?;
    let reciever_account = next_account_info(accounts_iter)?;

    // Check that instruction_data has at least 4 bytes
    if instruction_data.len() < 4 {
        return Err(ProgramError::InvalidInstructionData);
    }

    let amount = u32::from_le_bytes(instruction_data[0..4].try_into().unwrap());

    let mut sender_acc = TokenAccount::try_from_slice(&sender_account.data.borrow())?;
    let mut reciever_acc = TokenAccount::try_from_slice(&reciever_account.data.borrow())?;

    if sender_acc.balance < amount {
        return Err(solana_program::program_error::ProgramError::InsufficientFunds);
    }

    sender_acc.balance -= amount;
    reciever_acc.balance += amount;

    sender_acc.serialize(&mut &mut sender_account.data.borrow_mut()[..])?;
    reciever_acc.serialize(&mut &mut reciever_account.data.borrow_mut()[..])?;

    Ok(())
}
