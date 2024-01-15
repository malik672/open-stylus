//! Rust implementation of the Ownable contract.
//! Warning: this code has not been audited.
//! Does not check for zero address as parameter

use alloy_primitives::Address;
use alloy_sol_types::{sol, SolError};
use stylus_sdk::{
    evm, msg,
    stylus_proc::{external, sol_storage},
};

//STRUCTS
sol_storage! {
    pub struct Ownable {
        //Owners address
        address owner;
        //flag to check if address has been set
        bool check;

    }
}

//ERRORS
pub enum OwnableErrors {
    OwnableUnauthorizedAccount(OwnableUnauthorizedAccount),
    OwnableInvalidOwner(OwnableInvalidOwner),
    AlreadyConstructed(AlreadyConstructed),
}

impl From<OwnableErrors> for Vec<u8> {
    fn from(err: OwnableErrors) -> Vec<u8> {
        match err {
            OwnableErrors::AlreadyConstructed(e) => e.encode(),
            OwnableErrors::OwnableUnauthorizedAccount(e) => e.encode(),
            OwnableErrors::OwnableInvalidOwner(e) => e.encode(),
        }
    }
}

//EVENTS
sol! {
    event SetOwner(address indexed owner);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
}

//ERRORS
sol! {
    error OwnableUnauthorizedAccount();
    error OwnableInvalidOwner();
    error AlreadyConstructed();
}

impl Ownable {
    pub fn onlyOwner(&self) -> Result<(), OwnableErrors> {
        if *self.owner != msg::sender() {
            return Err(OwnableErrors::OwnableInvalidOwner(OwnableInvalidOwner {}));
        }
        Ok(())
    }

    pub fn constructor(&mut self, owner: Address) -> Result<(), OwnableErrors> {
        if *self.check {
            return Err(OwnableErrors::AlreadyConstructed(AlreadyConstructed {}));
        }
        evm::log(SetOwner {
            owner: msg::sender(),
        });
        Ok(self.owner.set(owner))
    }

    pub fn transferOwnership(&mut self, new_owner: Address) -> Result<(), OwnableErrors> {
        self.onlyOwner()?;

        evm::log(OwnershipTransferred {
            previousOwner: msg::sender(),
            newOwner: Address(*new_owner),
        });
        Ok(self.owner.set(new_owner))
    }
}
