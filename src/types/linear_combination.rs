use crate::ffi;
use ark_bn254::Fr;
use ark_ff::PrimeField;
use libc::size_t;

use super::GmpBigInt;

pub struct LinearCombination {
    pub(crate) inner: *mut ffi::LinearCombinationT,
}

impl LinearCombination {
    pub fn new(items: &[(usize, Fr)]) -> Self {
        let len = items.len();
        let nums: Vec<usize> = items.iter().map(|x| x.0).collect();
        let big_ints: Vec<GmpBigInt> = items
            .into_iter()
            .map(|x| GmpBigInt::from_bigint(x.1.into_bigint()))
            .collect();

        unsafe {
            let lc_ptr =
                ffi::make_linear_combination(len as size_t, nums.as_ptr(), big_ints.as_ptr());
            LinearCombination { inner: lc_ptr }
        }
    }
}

impl Drop for LinearCombination {
    fn drop(&mut self) {
        unsafe {
            ffi::clear_linear_combination(self.inner);
        }
    }
}
