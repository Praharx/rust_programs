use borsh::BorshDeserialize;
use solana_program::pubkey::Pubkey;
use solana_program::{
    entrypoint::ProgramResult,
    account_info::AccountInfo,
    program_error::ProgramError
};

use crate::instructions;
use crate::state::PageVisits;
use crate::state::IncrementPageVisits;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    if let Ok(page_visits) = PageVisits::try_from_slice(instruction_data) {
        return instructions::create::create_page_visits(program_id, accounts, page_visits);
    };

    if IncrementPageVisits::try_from_slice(instruction_data).is_ok() {
        return instructions::increment::increment_page_visits(accounts);
    };

    Err(ProgramError::InvalidInstructionData)
}