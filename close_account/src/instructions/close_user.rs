use solana_program::{
    account_info::{ next_account_info, AccountInfo}, 
    entrypoint::ProgramResult,
    sysvar::Sysvar,
    rent::Rent
};

pub fn close_user(
    accounts: &[AccountInfo]
) -> ProgramResult {
    // getting the accounts
    let account_iter = &mut accounts.iter();
    let target_account = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    // length of account
    let account_span = 0usize;
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    // getting the difference
    let diff = target_account.lamports() - lamports_required;

    // Sent the rent back to payer
    **target_account.lamports.borrow_mut() -= diff;
    **payer.lamports.borrow_mut() += diff;

    // Reallocating the account to zero
    target_account.realloc(account_span, true)?;

    // Assigning the account to system program
    target_account.assign(system_program.key);

    Ok(())
}