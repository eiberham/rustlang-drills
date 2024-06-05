use borsh_derive::{BorshDeserialize, BorshSerialize};
use solana_program::{
    msg,
    entrypoint,
    account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, pubkey::Pubkey,
    program_error::ProgramError
};

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Score {
    pub id: usize,
    pub value: usize
}

// declare and export the program's entrypoint
entrypoint!(process_instruction);
 
// program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // log a message to the blockchain

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    
    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId)
    }

    let mut data = Score::try_from_slice(&instruction_data).unwrap();
    data.value += 1;
    data.serialize(&mut &mut account.data.borrow_mut()[..])?;
    // gracefully exit the program
    msg!("Score updated successfully!");
    msg!("New score: {}", data.value);
    Ok(())
}
