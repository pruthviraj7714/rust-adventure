use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WalletAccount {
    balance: u8,
}

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let from_account = next_account_info(accounts_iter)?;
    let to_account = next_account_info(accounts_iter)?;

    let amount = u8::from_le_bytes(
        instruction_data
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    let result = invoke(
        &system_instruction::transfer(from_account.key, to_account.key, amount.into()),
        &[from_account.clone(), to_account.clone()],
    )?;

    msg!(
        "Transferred {} lamports from {} to {}.",
        amount,
        from_account.key,
        to_account.key
    );

    Ok(())
}
