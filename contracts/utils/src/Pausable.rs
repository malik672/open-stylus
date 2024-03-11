//! Rust implementation of the Pausable .
//! Warning: this code has not been audited.
//! Warning: Use at your risk
extern crate alloc;
use alloc::{vec::Vec, string::ToString};
use alloy_sol_types::sol;
use stylus_sdk::stylus_proc::{external, sol_storage};


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

impl ExpectedPause {
    fn encode(&self) -> Vec<u8> {
        // Convert the error to a string and then to bytes
        let error_str = "ExpectedPause".to_string();
        error_str.into_bytes()
    }
}

impl EnforcedPaused {
    fn encode(&self) -> Vec<u8> {
        // Convert the error to a string and then to bytes
        let error_str = "EnforcedPaused".to_string();
        error_str.into_bytes()
    }
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
