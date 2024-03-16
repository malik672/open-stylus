//! Rust implementation of the Pausable .
//! Warning: this code has not been audited.
//! Warning: Use at your risk
extern crate alloc;
use super::Errors::{Errors, FailedCall, InsufficientBalance};
use alloc::{format, string::ToString, vec::Vec};
use alloy_sol_types::sol;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    stylus_proc::{entrypoint, external, sol_storage},
    types::AddressVM,
};

sol_storage! {
  pub struct Nonces {
    mapping(address => uint256)  _nonces;
  }
}

impl Nonces {
    pub fn nonces(&self, owner: Address) -> U256 {
        let res = self._nonces.get(owner);
        res
    }

    pub fn _use_nonce(&mut self, owner: Address) -> U256 {
        let mut res = self._nonces.get(owner);
        res = res + U256::from(1);
        self._nonces.insert(owner, res);
        res
    }

    pub fn _use_checked_nonce(&mut self, owner: Address, nonce: U256) {
        let current = Self::_use_nonce(self, owner);

        if nonce != current {
            panic!("{}", format!("inavlid {} and {}", owner, current));
        }
    }
}
