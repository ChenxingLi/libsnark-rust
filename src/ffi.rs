use crate::types::GmpBigInt;
use libc::{c_void, size_t};

#[repr(C)]
pub struct LinearCombinationT(c_void);

#[repr(C)]
pub struct ConstraintSystemT(c_void);

#[repr(C)]
pub struct KeyPairT(c_void);

#[repr(C)]
pub struct ProofT(c_void);

#[repr(C)]
pub struct InputT(c_void);

extern "C" {
    pub fn new_constraint_system(
        primary_input_size: size_t,
        auxiliary_input_size: size_t,
    ) -> *mut ConstraintSystemT;
    pub fn add_constraint(
        cs: *mut ConstraintSystemT,
        A: *const LinearCombinationT,
        B: *const LinearCombinationT,
        C: *const LinearCombinationT,
    );
    pub fn constraint_satisfied(
        cs: *const ConstraintSystemT,
        primary_input: *const InputT,
        auxiliary_input: *const InputT,
    ) -> bool;
    pub fn make_linear_combination(
        len: size_t,
        nums: *const size_t,
        big_ints: *const GmpBigInt,
    ) -> *mut LinearCombinationT;
    pub fn make_input(len: size_t, input: *const GmpBigInt) -> *mut InputT;
    pub fn setup(cs: *mut ConstraintSystemT) -> *mut KeyPairT;
    pub fn prove(
        keypair: *const KeyPairT,
        primary_input: *const InputT,
        auxiliary_input: *const InputT,
    ) -> *mut ProofT;
    pub fn verify(
        keypair: *const KeyPairT,
        primary_input: *const InputT,
        proof: *const ProofT,
    ) -> bool;
    pub fn clear_linear_combination(lc: *mut LinearCombinationT);
    pub fn clear_constraint_system(cs: *mut ConstraintSystemT);
    pub fn clear_key_pair(keyPair: *mut KeyPairT);
    pub fn clear_proof(proof: *mut ProofT);
    pub fn clear_input(input: *mut InputT);
    pub fn init_public_params();
    pub fn reset_profile();
    pub fn toggle_profile_log(enable: bool);
}
