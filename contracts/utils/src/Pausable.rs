//! Rust implementation of the Inspector Interface.
//! Warning: this code has not been audited.
//! Warning: Use at your risk

extern crate alloc;
use alloc::vec::Vec;
use alloy_sol_types::{sol, SolError};
use stylus_sdk::{
    evm, msg,
    stylus_proc::{external, sol_storage},
};

sol_storage! {
    ///STORAGE
    pub struct Pausable {
        ///flag to check if it's closed or open
        bool _paused;
    }
}

//EVENTS
sol! {
    event Paused(address account);
    event Unpaused(address account);
}

//ERRORS
sol! {
    error EnforcedPaused();
    error ExpectedPause();
}

//ERRORS
pub enum PausableErrors {
    EnforcedPaused(EnforcedPaused),
    ExpectedPause(ExpectedPause),
}

impl From<PausableErrors> for Vec<u8> {
    fn from(err: PausableErrors) -> Vec<u8> {
        match err {
            PausableErrors::EnforcedPaused(e) => e.encode(),
            PausableErrors::ExpectedPause(e) => e.encode(),
        }
    }
}

#[external]
impl Pausable {
    pub fn paused(&self) -> Result<bool, PausableErrors> {
        Ok(self._paused.get())
    }

    pub fn _requireNotPaused(&self) -> Result<(), PausableErrors> {
        //unwrap used since there's little to no possibility for failure
        if self.paused().ok().unwrap() {
            return Err(PausableErrors::EnforcedPaused(EnforcedPaused {}));
        }
        Ok(())
    }

    pub fn _requirePaused(&self) -> Result<(), PausableErrors> {
        if !self.paused().ok().unwrap() {
            return Err(PausableErrors::ExpectedPause(ExpectedPause {}));
        }
        Ok(())
    }

    fn _pause(&mut self) -> Result<(), PausableErrors> {
        self._requireNotPaused()?;
        self._paused.set(true);
        Ok(())
    }

    fn _unpause(&mut self) -> Result<(), PausableErrors> {
        self._requirePaused()?;
        self._paused.set(false);
        Ok(())
    }
}
