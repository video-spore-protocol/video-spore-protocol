use alloc::{vec::Vec};
use core::result::Result;

use ckb_std::ckb_constants::Source::{Input, Output};
use ckb_std::high_level::{load_script};
use ckb_std::{
    ckb_types::prelude::*,
    ckb_types::packed::*,
    high_level::{load_cell_type, QueryIter},
};
use ckb_std::error::SysError;

/// Error
#[repr(i8)]
pub enum Error {
    IndexOutOfBound = 1,
    ItemMissing,
    LengthNotEnough,
    Encoding,

    ZeroInputSporeMelt = 101,
    NonZeroOutputSporeMelt = 102,
}

impl From<SysError> for Error {
    fn from(err: SysError) -> Self {
        use SysError::*;
        match err {
            IndexOutOfBound => Self::IndexOutOfBound,
            ItemMissing => Self::ItemMissing,
            LengthNotEnough(_) => Self::LengthNotEnough,
            Encoding => Self::Encoding,
            Unknown(err_code) => panic!("unexpected sys error {}", err_code),
        }
    }
}


pub fn main() -> Result<(), Error> {

    // The implementation method for Video Spore Protocol using a
    // binding-lifecycle script involves placing the type_hash of the Spore Cell
    // into the lock_args of the Spore Segment Cell. This approach enables Spore
    // Cells to be filtered using the type_script's type_hash.

    let spore_type_hash_bytes = load_script()?.args();
    let spore_type_hash = Byte32::new_unchecked(spore_type_hash_bytes.raw_data());

    let spore_input: Vec<_> = QueryIter::new(load_cell_type, Input)
        .filter(|script_opt| match script_opt {
            None => false,
            Some(script) => script.calc_script_hash() == spore_type_hash
        })
        .collect();
    if spore_input.len() == 0 {
        return Err(Error::ZeroInputSporeMelt);
    }

    let spore_output: Vec<_> = QueryIter::new(load_cell_type, Output)
        .filter(|script_opt| match script_opt {
            None => false,
            Some(script) => script.calc_script_hash() == spore_type_hash
        })
        .collect();
    if spore_output.len() > 0 {
        return Err(Error::NonZeroOutputSporeMelt);
    }

    return Ok(());
}
