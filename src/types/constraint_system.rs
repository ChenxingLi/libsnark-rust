use libc::size_t;

use crate::ffi;

use super::{Input, LinearCombination};

pub struct ConstraintSystem {
    pub(crate) inner: *mut ffi::ConstraintSystemT,
}

impl ConstraintSystem {
    pub fn new(primary_input_size: usize, auxiliary_input_size: usize) -> Self {
        unsafe {
            let cs_ptr = ffi::new_constraint_system(
                primary_input_size as size_t,
                auxiliary_input_size as size_t,
            );
            ConstraintSystem { inner: cs_ptr }
        }
    }

    pub fn add_constraint(
        &mut self,
        a: &LinearCombination,
        b: &LinearCombination,
        c: &LinearCombination,
    ) {
        unsafe {
            ffi::add_constraint(self.inner, a.inner, b.inner, c.inner);
        }
    }

    pub fn is_satisfied(&self, primary_input: &Input, auxiliary_input: &Input) -> bool {
        unsafe { ffi::constraint_satisfied(self.inner, primary_input.inner, auxiliary_input.inner) }
    }
}

impl Drop for ConstraintSystem {
    fn drop(&mut self) {
        unsafe {
            ffi::clear_constraint_system(self.inner);
        }
    }
}
