#![no_std]
#![cfg_attr(not(test), no_main)]

// define modules
mod entry;

#[cfg(test)]
extern crate alloc;

#[cfg(not(test))]
use ckb_std::default_alloc;
#[cfg(not(test))]
ckb_std::entry!(program_entry);
#[cfg(not(test))]
default_alloc!(6 * 1024, 3000 * 1024, 64);

/// program entry
pub fn program_entry() -> i8 {
    match entry::main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}
