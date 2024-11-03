use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::borrow::BorrowMut;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TreasuryAccount {
    fee: u32,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UserAccount {
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
    let receiver_account = next_account_info(accounts_iter)?;
    let treasury_account = next_account_info(accounts_iter)?;

    // Parse the transfer amount from instruction_data
    let amount = match instruction_data.get(0..4) {
        Some(data) => u32::from_le_bytes(
            data.try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        ),
        None => return Err(ProgramError::InvalidInstructionData),
    };

    // Deserialize account data for sender, receiver, and treasury
    let mut sender_account_data = UserAccount::try_from_slice(&sender_account.data.borrow())?;
    let mut receiver_account_data = UserAccount::try_from_slice(&receiver_account.data.borrow())?;
    let mut treasury_account_data =
        TreasuryAccount::try_from_slice(&treasury_account.data.borrow())?;

    // Calculate the fee (2%) and check if sender has enough balance
    let fee = (amount * 2) / 100;
    let total_deduction = amount;

    if sender_account_data.balance < total_deduction {
        return Err(ProgramError::InsufficientFunds);
    }

    // Perform the transaction
    sender_account_data.balance -= total_deduction;
    receiver_account_data.balance += amount - fee;
    treasury_account_data.fee += fee;

    // Serialize the updated account data back to accounts

    // Serialize sender account data
    sender_account_data.serialize(&mut &mut sender_account.data.borrow_mut())?;

    // Serialize receiver account data
    receiver_account_data.serialize(&mut &mut receiver_account.data.borrow_mut())?;

    // Serialize treasury account data
    treasury_account_data.serialize(&mut &mut treasury_account.data.borrow_mut())?;

    Ok(())
}
