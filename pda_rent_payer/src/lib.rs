use solana_program::entrypoint;

pub use processor::process_instruction;

pub mod state;
pub mod instructions;
pub mod processor;

entrypoint!(process_instruction);