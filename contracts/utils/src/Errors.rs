extern crate alloc;
use alloc::{string::ToString, vec::Vec};
use alloy_sol_types::sol;


//ERRORS
sol! {
    error InsufficientBalance(uint256 balance, uint256 needed);
    error FailedCall();
    error FailedDeployment();
}

//ERRORS
pub enum Errors {
    InsufficientBalance(InsufficientBalance),
    FailedCall(FailedCall),
    FailedDeployment(FailedDeployment),
}


impl InsufficientBalance {
    fn encode(&self) -> Vec<u8> {
        // Convert the error to a string and then to bytes
        let error_str = "InsufficientBalance".to_string();
        error_str.into_bytes()
    }
}

impl FailedCall {
    fn encode(&self) -> Vec<u8> {
        // Convert the error to a string and then to bytes
        let error_str = "FailedCall".to_string();
        error_str.into_bytes()
    }
}
impl FailedDeployment {
    fn encode(&self) -> Vec<u8> {
        // Convert the error to a string and then to bytes
        let error_str = "FailedDeployment".to_string();
        error_str.into_bytes()
    }
}

impl From<Errors> for Vec<u8> {
    fn from(err: Errors) -> Vec<u8> {
        match err {
            Errors::InsufficientBalance(e) => e.encode(),
            Errors::FailedCall(e) => e.encode(),
            Errors::FailedDeployment(e) => e.encode(),
        }
    }
}