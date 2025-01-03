use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
};

// Define a simple directive
#[derive(Debug, Clone)]
pub enum HelloWorldInstruction {
    SayHello,
}

// Parsing instruction data
fn parse_instruction(instruction_data: &[u8]) -> Result<HelloWorldInstruction, ProgramError> {
    let instruction = &instruction_data[0];
    match instruction {
        0 => Ok(HelloWorldInstruction::SayHello),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

// Program entry point
entrypoint!(process_instruction);
pub fn process_instruction(
    _program_id: &solana_program::pubkey::Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = parse_instruction(instruction_data)?;
    match instruction {
        HelloWorldInstruction::SayHello => {
            msg!("successful!");
            Ok(())
        }
    }
}
