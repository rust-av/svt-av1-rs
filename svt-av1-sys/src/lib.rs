#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[cfg_attr(feature = "cargo-clippy", allow(const_static_lifetime))]
#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]

pub mod SvtAV1Sys {
    include!(concat!(env!("OUT_DIR"), "/svtav1.rs"));
}

pub use SvtAV1Sys::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;
    use std::ptr;
    #[test]
    fn encoder_setup() {
        unsafe {
            let mut handle: *mut EbComponentType = ptr::null_mut();
            let mut cfg = mem::MaybeUninit::zeroed();

            let ret = svt_av1_enc_init_handle(&mut handle, ptr::null_mut(), cfg.as_mut_ptr());
            assert_eq!(ret, EbErrorType::EB_ErrorNone);

            let mut cfg = cfg.assume_init();

            cfg.source_width = 64;
            cfg.source_height = 64;

            let ret = svt_av1_enc_set_parameter(handle, &mut cfg);
            assert_eq!(ret, EbErrorType::EB_ErrorNone);

            let ret = svt_av1_enc_init(handle);
            assert_eq!(ret, EbErrorType::EB_ErrorNone);
        }
    }
}
