mod bigint;
mod constraint_system;
mod input;
mod linear_combination;

pub use bigint::GmpBigInt;
pub use constraint_system::ConstraintSystem;
pub use input::Input;
pub use linear_combination::LinearCombination;

use crate::ffi;

pub struct KeyPair {
    pub(crate) inner: *mut ffi::KeyPairT,
}

impl Drop for KeyPair {
    fn drop(&mut self) {
        unsafe {
            ffi::clear_key_pair(self.inner);
        }
    }
}

pub struct Proof {
    pub(crate) inner: *mut ffi::ProofT,
}

impl Drop for Proof {
    fn drop(&mut self) {
        unsafe {
            ffi::clear_proof(self.inner);
        }
    }
}
