use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    program_pack::{IsInitialized, Pack, Sealed},
};

// Each NFT is 47 bytes -> (32 + 16 + (8 * 32) + (32 * 2) + 8) / 8
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Exotic {
    pub id: u32,
    pub ref_id: u16,
    pub owner: Pubkey,
    pub strain1: u32,
    pub strain2: u32,
    pub burned: u8,
}

impl Sealed for Exotic { }

impl IsInitialized for Exotic {
    fn is_initialized(&self) -> bool {
        self.is_initialized()
    }
}

impl Pack for Exotic {
    const LEN: usize = 47;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 47];
        let (id, ref_id, owner, strain1, strain2, burned) = array_refs![src, 4, 2, 32, 4, 4, 1];
        
        Ok(Exotic {
            id: u32::from_le_bytes(*id),
            ref_id: u16::from_le_bytes(*ref_id),
            owner: Pubkey::new_from_array(*owner),
            strain1: u32::from_le_bytes(*strain1),
            strain2: u32::from_le_bytes(*strain2),
            burned: burned[0],
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 47];
        let (
            id_dst,
            ref_id_dst,
            owner_dst,
            strain1_dst,
            strain2_dst,
            burned_dst,
        ) = mut_array_refs![dst, 4, 2, 32, 4, 4, 1];
        
        let Exotic {
            id,
            ref_id,
            ref owner,
            strain1,
            strain2,
            burned,
        } = self;
        

        *id_dst = id.to_le_bytes();
        *ref_id_dst = ref_id.to_le_bytes();
        owner_dst.copy_from_slice(owner.as_ref());
        *strain1_dst = strain1.to_le_bytes();
        *strain2_dst = strain2.to_le_bytes();
        burned_dst[0] = *burned as u8;
    }
}