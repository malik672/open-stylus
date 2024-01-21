//! Rust implementation of the Inspector Interface.
//! Warning: this code has not been audited.
//! Warning: Use at your risk

extern crate alloc;

use stylus_sdk::stylus_proc::sol_storage;

//STRUCTS
sol_storage! {
pub struct Erc165{
    bytes4 interfaceId;
  }
}

impl Erc165 {
    pub fn supports_interface(&mut self, bytes_id: [u8; 4]) -> bool {
        return self.interfaceId.get() == bytes_id;
    }
}
