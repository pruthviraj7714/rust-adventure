use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    entrypoint,
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorDataAccount {
    pub result: i32,
}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut calculator_data = CalculatorDataAccount::try_from_slice(&account.data.borrow())?;

    let operation = instruction_data[0];

    let operand = i32::from_le_bytes(
        instruction_data[1..4]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    match operation {
        0 => calculator_data.result += operand,
        1 => calculator_data.result -= operand,
        2 => calculator_data.result *= operand,
        3 => calculator_data.result /= operand,
        _ => return Err(ProgramError::InvalidInstructionData),
    }

    calculator_data.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
