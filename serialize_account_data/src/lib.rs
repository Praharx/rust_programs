use solana_program::entrypoint;

pub mod instructions;
pub mod state;
pub mod processor;

pub use processor::process_instruction;

entrypoint!(process_instruction);

