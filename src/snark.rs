use crate::ffi;
use crate::types::{ConstraintSystem, Input, KeyPair, Proof};

pub fn setup(cs: &ConstraintSystem) -> KeyPair {
    unsafe {
        let kp_ptr = ffi::setup(cs.inner);
        KeyPair { inner: kp_ptr }
    }
}

pub fn prove(keypair: &KeyPair, primary_input: &Input, auxiliary_input: &Input) -> Proof {
    unsafe {
        let proof_ptr = ffi::prove(keypair.inner, primary_input.inner, auxiliary_input.inner);
        Proof { inner: proof_ptr }
    }
}

pub fn verify(keypair: &KeyPair, primary_input: &Input, proof: &Proof) -> bool {
    unsafe { ffi::verify(keypair.inner, primary_input.inner, proof.inner) }
}
