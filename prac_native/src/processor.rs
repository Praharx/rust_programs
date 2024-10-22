use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    pubkey::Pubkey
};

use crate::instructions::{close_user::close_user, create_user::create_user};
use crate::state::User;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Instruction{
    CreateUser(User),
    CloseUser,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8]
) -> ProgramResult {
    let instruction = Instruction::try_from_slice(input)?;
    match instruction {
        Instruction::CreateUser(data) => create_user(program_id, accounts, data),
        Instruction::CloseUser => close_user(accounts)
    }
}
