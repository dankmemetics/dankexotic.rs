use crate::{error::ExoticError, processor::Processor};

use solana_program::{
    account_info::AccountInfo, entrypoint, msg, entrypoint::ProgramResult,
    program_error::PrintProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, instruction_data) {
        return Err(error);
    }
    Ok(())
}