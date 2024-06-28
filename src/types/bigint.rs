use ark_ff::BigInt;
use gmp_mpfr_sys::gmp::mpz_t;
use gmp_mpfr_sys::gmp::*;

use std::mem::MaybeUninit;

#[repr(transparent)]
pub struct GmpBigInt {
    inner: mpz_t,
}
const ENDIAN: i8 = if cfg!(target_endian = "big") { 1 } else { -1 };

impl GmpBigInt {
    pub fn from_bigint<const N: usize>(bigint: BigInt<N>) -> Self {
        unsafe {
            let mut z = MaybeUninit::uninit();
            mpz_init(z.as_mut_ptr());
            let mut z = z.assume_init();

            // Step 3: Set the value of mpz_t using the byte array
            mpz_import(
                &mut z,
                N,
                -1,
                8,
                ENDIAN as i32,
                0,
                bigint.0.as_ptr() as *const _,
            );

            GmpBigInt { inner: z }
        }
    }
}

impl Drop for GmpBigInt {
    fn drop(&mut self) {
        unsafe {
            mpz_clear(&mut self.inner);
        }
    }
}
