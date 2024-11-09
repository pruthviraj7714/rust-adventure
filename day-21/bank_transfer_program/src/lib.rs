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
pub struct BankAccount {
    pub balance: u64,
}

// Program entry point
entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let sender_account = next_account_info(accounts_iter)?;
    let receiver_account = next_account_info(accounts_iter)?;

    let amount = u64::from_le_bytes(
        instruction_data[0..8]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    let mut sender_bank_account = BankAccount::try_from_slice(&sender_account.data.borrow())?;
    let mut receiver_bank_account = BankAccount::try_from_slice(&receiver_account.data.borrow())?;

    if sender_bank_account.balance < amount {
        msg!("Insufficient funds in sender's account.");
        return Err(ProgramError::InsufficientFunds);
    }

    sender_bank_account.balance -= amount;
    receiver_bank_account.balance += amount;

    sender_bank_account.serialize(&mut &mut sender_account.data.borrow_mut()[..])?;
    receiver_bank_account.serialize(&mut &mut receiver_account.data.borrow_mut()[..])?;

    Ok(())
}