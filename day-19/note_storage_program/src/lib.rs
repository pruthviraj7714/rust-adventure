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
pub struct NoteAccount {
    note: String,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if !account.is_writable || account.owner != program_id {
        return Err(ProgramError::InvalidAccountData);
    }

    let command = instruction_data[0];
    let new_note = String::from_utf8(instruction_data[1..].to_vec()).map_err(|_| ProgramError::InvalidInstructionData)?;

    let mut note_account = NoteAccount::try_from_slice(&account.data.borrow())?;

    if command == 0 {
        note_account.note = new_note;
        note_account.serialize(&mut &mut account.data.borrow_mut())?;
    } else if command == 1 {
        msg!("The Note in the account is: {}", note_account.note);
    } else {
        return Err(ProgramError::InvalidInstructionData);
    }

    Ok(())
}