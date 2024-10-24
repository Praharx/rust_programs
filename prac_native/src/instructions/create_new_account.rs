use solana_program::{
    entrypoint::ProgramResult,
    account_info::{ AccountInfo, next_account_info},
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

use crate::state::RentVault;

pub fn create_new_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo]
) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let new_account = next_account_info(account_iter)?;
    let rent_vault = next_account_info(account_iter)?;
    let _system_program = next_account_info(account_iter)?;

    // creating a new pda
    let (rent_vault_pda, rent_vaut_bump) = 
    Pubkey::find_program_address(&[RentVault::SEED_PREFIX.as_bytes()], program_id);
    assert!(rent_vault.key.eq(&rent_vault_pda));

    // no data in the account
    let lamports_required = (Rent::get()?).minimum_balance(0);

    // transferring the min. lamports required.
    **rent_vault.lamports.borrow_mut() -= lamports_required;
    **new_account.lamports.borrow_mut() += lamports_required;

    Ok(())
}