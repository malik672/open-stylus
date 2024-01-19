//! Rust implementation of the Inspector Interface.
//! Warning: this code has not been audited.
//! Warning: Use at your risk

extern crate alloc;

use stylus_sdk::stylus_proc::sol_interface;

sol_interface! {
    interface IOwnable {
     //Can only be called by owner
     function supportsInterface(bytes4 InterfaceId) external view returns(bool);
    }
}
