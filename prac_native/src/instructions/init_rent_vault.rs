use solana_program::{
    entrypoint::ProgramResult,
    account_info::{ AccountInfo, next_account_info },
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};
use borsh::{ BorshDeserialize, BorshSerialize };

use crate::state::RentVault;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct InitRentVaultArgs {
    fund_lamports: u64,
}

pub fn init_rent_vault(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: InitRentVaultArgs
) -> ProgramResult {
    // unwrapping the accounts
    let account_iter = &mut accounts.iter();
    let rent_vault = next_account_info(account_iter)?;
    let payer = next_account_info(account_iter)?;
    let system_program = next_account_info(account_iter)?;

    // finding the pda
    let (rent_vault_pda, rent_vault_bump) = 
        Pubkey::find_program_address(&[RentVault::SEED_PREFIX.as_bytes()], program_id);
    // since the pda was derived on client side and then passed as refernce we can put an assert to check if they are the same.
    assert!(rent_vault.key.eq(&rent_vault_pda));

    // calculating the rent
    let lamports_required = (Rent::get()?).minimum_balance(0) + args.fund_lamports;

    // initiating the pda account, transferring the initial lamports & also it to program_id.
    invoke_signed(
        &system_instruction::create_account(
            payer.key, 
            rent_vault.key,
            lamports_required, 
            0, 
            program_id
        ),
        &[payer.clone(), rent_vault.clone(), system_program.clone()],
        &[&[RentVault::SEED_PREFIX.as_bytes(), &[rent_vault_bump]]]
    )?;

    Ok(())
}