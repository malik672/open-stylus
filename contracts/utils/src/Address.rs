//! Rust implementation of the Pausable .
//! Warning: this code has not been audited.
//! Warning: Use at your risk
extern crate alloc;
use super::Errors::{Errors, FailedCall, InsufficientBalance};
use alloc::{string::ToString, vec::Vec};
use alloy_sol_types::sol;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    call::{self, delegate_call, static_call, transfer_eth, Call},
    contract::balance,
    evm, msg,
    stylus_proc::{external, sol_storage}, types::AddressVM,
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

impl ADDRESSES {
    fn send_value(recipient: Address, amount: U256) -> Result<(), Errors> {
        if balance() < amount {
            return Err(Errors::InsufficientBalance(InsufficientBalance {
                balance: balance(),
                needed: amount,
            }));
        }
        let _ = transfer_eth(recipient, amount);
        Ok(())
    }

    fn function_call(target: Address, data: Vec<u8>) -> Result<(), Errors> {
        Self::function_call_with_value(target, data, U256::from(0))
    }

    fn function_call_with_value(target: Address, data: Vec<u8>, value: U256) -> Result<(), Errors> {
        if balance() < value {
            return Err(Errors::InsufficientBalance(InsufficientBalance {
                balance: balance(),
                needed: value,
            }));
        }
        // let returndata = call::call(90, target, data.as_slice());
        Ok(())
    }

    fn function_static_call(target: Address, data: Vec<u8>) -> Vec<u8> {
        //    let retdata = static_call(context, target, data.as_slice())?
        todo!()
    }

    fn function_delegate_call(target: Address, data: Vec<u8>) -> Vec<u8> {
        
        // let return_data =unsafe { delegate_call(context, target, data.as_slice())};
        // return_data.unwrap();
        todo!()
    }

    fn verify_call_result_from_target(target: Address, success: bool, data: Vec<u8>) -> Vec<u8> {
        if !success {
            panic!("call not successful");
        }else {
            if data.len() == 0 && target.has_code() == false {
                panic!("either target address has no code or data is equal to zero")
            }
            data
        }
    }

    fn verify_call_result(success: bool, data: Vec<u8>) -> Vec<u8> {
        if !success {
            panic!("wrong");
        }else {
            data
        }
    }

    fn _revert(data: Vec<u8>) -> Result<(), Errors>{
        if data.len() > 0 {
            panic!("return data size should")
        }else {
           return Err(Errors::FailedCall(FailedCall {}));
        }
    }
}
