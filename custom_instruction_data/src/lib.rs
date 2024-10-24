use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    // deserialising the instruction data
    let instruction_data_object = InstructionData::try_from_slice(instruction_data)?;

    msg!("Welcome to the park {}", instruction_data_object.name);
    if instruction_data_object.height > 5 {
        msg!("You are tall enough to ride this bike. Congrats!");
    } else {
        msg!("Sorry you aint that tall!");
    };

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize ,Debug)]
pub struct InstructionData {
    name: String,
    height: u32
}

