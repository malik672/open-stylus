//! Rust implementation of the Pausable .
//! Warning: this code has not been audited.
//! Warning: Use at your risk
extern crate alloc;
use super::Errors::{Errors, FailedCall, InsufficientBalance};
use alloc::{format, string::ToString, vec::Vec};
use alloy_sol_types::sol;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    evm, msg,
    stylus_proc::{entrypoint, external, sol_storage},
    types::AddressVM,
};

pub struct CONTEXT;

impl CONTEXT {
    pub fn _msg_sender() -> Address {
        msg::sender()
    }

    pub fn _msg_data() -> U256 {
        msg::value()
    }

    pub fn  _context_suffx_length() -> U256 {
        U256::from(0)
    }
}
