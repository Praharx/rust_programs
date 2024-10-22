use borsh::BorshSerialize;
use solana_program::{
    account_info::{ 
        next_account_info, AccountInfo},
        entrypoint::ProgramResult, 
        program::invoke_signed, 
        pubkey::Pubkey, 
        rent::Rent, 
        system_instruction, 
        sysvar::Sysvar
};

use crate::state::User;

pub fn create_user(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: User
) -> ProgramResult {
    // get out all the accounts
    let account_iter = &mut accounts.iter();
    let target_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_account = next_account_info(account_iter)?;

    // calculate the length of the data
    let account_span = (data.try_to_vec()?).len();
    let lamports = (Rent::get()?).minimum_balance(account_span);

    // getting the bumps
    let (_,bump) = Pubkey::find_program_address(
        &[User::SEED_PREFIX.as_bytes(), payer.key.as_ref()],
        program_id
    );

    // writing the instruction
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            target_account.key,
            lamports,
            account_span as u64,
            program_id
        ),
        &[
            payer.clone(),
            target_account.clone(),
            system_account.clone(),
        ],
        &[&[User::SEED_PREFIX.as_bytes(), payer.key.as_ref(), &[bump]]],
    )?;

    // serializing the data to send to the blockchain
    data.serialize(&mut &mut target_account.try_borrow_mut_data()?.as_mut())?;
    Ok(())
}
    
