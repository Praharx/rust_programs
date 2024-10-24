use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey
};
use borsh::{ BorshDeserialize, BorshSerialize };

use crate::instructions::{
    create_new_account::create_new_account,
    init_rent_vault::{init_rent_vault, InitRentVaultArgs}
};


#[derive(BorshDeserialize, BorshSerialize)]
pub enum MyInstruction {
    InitRentVault(InitRentVaultArgs),
    CreateNewAccount,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    // deserialising the instruction 
    let instruction = MyInstruction::try_from_slice(input)?;

    match instruction {
        MyInstruction::InitRentVault(args) => init_rent_vault(program_id, accounts, args),
        MyInstruction::CreateNewAccount => create_new_account(program_id, accounts),
    }
}