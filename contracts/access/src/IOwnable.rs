//! Rust implementation of the Ownable Interface.
//! Warning: this code has not been audited.
//! Warning: Use at your risk

extern crate alloc;

use stylus_sdk::stylus_proc::sol_interface;

sol_interface! {
    interface IOwnable {

     //triggers when some unauthorized tries to access the function.
     error OwnableUnauthorizedAccount();
     //triggers when some unauthorized tries to access the function.
     error OwnableInvalidOwner();
     //triggers when code has been initialized
     error AlreadyConstructed();

     // Emitted during constrcution time when a `new owner` is selected
     event SetOwner(address indexed owner);
     
     // Emitted when ownership is transferred to another address
     event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

     //Can only be called by owner
     function transferOwnership(address newOwner) external;
    }
}
