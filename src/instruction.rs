use crate::{error::ExoticError};

use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_option::COption,
    pubkey::Pubkey,
    sysvar,
};
use std::convert::TryInto;
use std::mem::size_of;

#[repr(C)]
#[derive(Clone, Debug, PartialEq)]

pub enum ExoticInstruction {
    MintExotic {
        ref_id: u16,
        owner: Pubkey,
    },

    BreedExotics {
        strain1: u32,
        strain2: u32,
    },
}

impl ExoticInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use ExoticError::InvalidInstruction;

        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => {
                let (ref_id, rest) = rest.split_at(2);
                let ref_id = ref_id
                    .try_into()
                    .ok()
                    .map(u16::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                let (owner, rest) = Self::unpack_pubkey(rest)?;

                Self::MintExotic {
                    ref_id,
                    owner,
                }
            }

            1 => {
                let (strain1, rest) = rest.split_at(4);
                let strain1 = strain1
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                let (strain2, rest) = rest.split_at(4);
                let strain2 = strain2
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                Self::BreedExotics {
                    strain1,
                    strain2,
                }
            }
            _ => return Err(ExoticError::InvalidInstruction.into()),
        })
    }

    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() >= 32 {
            let (key, rest) = input.split_at(32);
            let pk = Pubkey::new(key);
            Ok((pk, rest))
        } else {
            Err(ExoticError::InvalidInstruction.into())
        }
    }
}