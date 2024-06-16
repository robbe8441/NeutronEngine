use std::ffi::CStr;

pub trait ConvertCStr {
    fn to_cstr_unchecked(&self) -> &CStr;
}

impl ConvertCStr for &str {
    fn to_cstr_unchecked(&self) -> &CStr {
        unsafe { CStr::from_bytes_with_nul_unchecked(self.as_bytes()) }
    }
}






