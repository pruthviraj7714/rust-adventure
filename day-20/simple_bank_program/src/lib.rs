use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError, pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BankAccount {
    pub balance: u64,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut bank_account_data = BankAccount::try_from_slice(&account.data.borrow())?;

    let instruction = instruction_data[0];

    let amount = u64::from_le_bytes(
        instruction_data[1..4]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    match instruction {
        //deposit
        0 => bank_account_data.balance += amount,
        //withdraw
        1 =>
            if bank_account_data.balance <= amount {
                return Err(ProgramError::InsufficientFunds);
            } else {
                bank_account_data.balance -= amount
            }
        //Show Balance
        2 => msg!("You have Current Balance : {}", bank_account_data.balance),
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    bank_account_data.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
