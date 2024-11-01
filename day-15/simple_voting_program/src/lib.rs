use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::{self, ProgramResult}, pubkey::Pubkey};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VotingAccount {
    yes_votes : u32,
    no_votes : u32   
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data : &[u8]
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut voting_account = VotingAccount::try_from_slice(&account.data.borrow())?;

    let vote_type = instruction_data[0];

    match vote_type {
        0 => voting_account.yes_votes += 1,
        1 => voting_account.no_votes += 1,
        _ => return Err(solana_program::program_error::ProgramError::InvalidInstructionData)
    }

    voting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}