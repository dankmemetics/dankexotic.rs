use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

use crate::instruction::ExoticInstruction;

pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = ExoticInstruction::unpack(instruction_data)?;

        match instruction {
            ExoticInstruction::MintExotic { ref_id, owner } => {
                msg!("Instruction: Mint Exotic");
                Ok(())
            }
            ExoticInstruction::BreedExotics { strain1, strain2 } => {
                msg!("Instruction: Breed Exotics");
                Ok(())
            }
        }
    }
}