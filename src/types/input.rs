use ark_bn254::Fr;
use ark_ff::PrimeField;
use libc::size_t;

use super::GmpBigInt;
use crate::ffi;

pub struct Input {
    pub(crate) inner: *mut ffi::InputT,
}

impl Input {
    pub fn new(input: &[GmpBigInt]) -> Self {
        let len = input.len();
        unsafe {
            let input_ptr = input.as_ptr();
            let input_t_ptr = ffi::make_input(len as size_t, input_ptr);
            Input { inner: input_t_ptr }
        }
    }

    pub fn from_fr(input: &[Fr]) -> Self {
        Self::new(
            &input
                .iter()
                .map(|x| GmpBigInt::from_bigint(x.into_bigint()))
                .collect::<Vec<_>>(),
        )
    }
}

impl Drop for Input {
    fn drop(&mut self) {
        unsafe {
            ffi::clear_input(self.inner);
        }
    }
}
