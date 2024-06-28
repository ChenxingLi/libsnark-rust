use crate::ffi;

pub fn init_public_params() {
    unsafe { ffi::init_public_params() }
}

pub fn reset_profile() {
    unsafe { ffi::reset_profile() }
}

pub fn toggle_profile_log(enable: bool) {
    unsafe { ffi::toggle_profile_log(enable) }
}
