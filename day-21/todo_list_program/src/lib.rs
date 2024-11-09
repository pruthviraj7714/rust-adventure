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
pub struct TodoList {
    tasks: Vec<String>,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut todo_list = TodoList::try_from_slice(&account.data.borrow())?;

    //add task
    if instruction_data[0] == 0 {
        todo_list.tasks.push(
            String::from_utf8(instruction_data[1..].to_vec())
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        );
        //Remove Task
    } else if instruction_data[0] == 1 {
        if !todo_list.tasks.is_empty() {
            todo_list.tasks.pop();
        } else {
            msg!("No More tasks to remove");
        }
    }

    todo_list.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
