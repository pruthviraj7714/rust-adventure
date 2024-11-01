use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CounterProgram {
    count : u32
}

entrypoint!(process_instruction)

fn process_instruction(
    _program_id : &Pubkey,
    accounts : &[AccountInfo],
    instruction_data : &[u8]
) -> ProgramResult {

    let account_iter = &mut accounts.iter();    
    let account = next_account_info(account_iter)?;

    let counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    if instruction_data[0] == 0 {
        counter_account.count += 1;
    }else if instruction_data[0] == 1 {
        counter_account.counter -= 1;
    } else {
        println!("Invalid Instruction");
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}






