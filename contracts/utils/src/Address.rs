//! Rust implementation of the Pausable .
//! Warning: this code has not been audited.
//! Warning: Use at your risk
extern crate alloc;
use alloc::{string::ToString, vec::Vec};
use alloy_sol_types::sol;
use stylus_sdk::{
    alloy_primitives::{Address, U256}, call::transfer_eth, contract::balance, evm, msg, stylus_proc::{external, sol_storage}
};

pub struct ADDRESSES;

//ERRORS
sol! {
    error AddressEmptyCode();
}

//ERRORS
pub enum AddressErrors {
    AddressEmptyCode(AddressEmptyCode),
}

impl AddressEmptyCode {
    fn encode(&self) -> Vec<u8> {
        // Convert the error to a string and then to bytes
        let error_str = "AddressEmptyCode".to_string();
        error_str.into_bytes()
    }
}

impl From<AddressErrors> for Vec<u8> {
    fn from(err: AddressErrors) -> Vec<u8> {
        match err {
            AddressErrors::AddressEmptyCode(e) => e.encode(),
        }
    }
}

#[external]
impl ADDRESSES {
    fn send_value(recipient: Address, amount: U256) -> Result<(), AddressErrors> {
        if balance() < amount {
          return  Err(AddressErrors::AddressEmptyCode(AddressEmptyCode {}));
        } 
        let _ = transfer_eth(recipient, amount);
        Ok(())
    }

    fn function_call(target: Address, data: Vec<u8>) -> Vec<u8> {
       todo!()
    }

    fn function_call_with_value(target: Address, data: Vec<u8>, value: U256) -> Result<(), AddressErrors> {
        if balance() < amount {
            return  Err(AddressErrors::AddressEmptyCode(AddressEmptyCode {
                   
            }));
          } 
          Ok(())
    }
}
