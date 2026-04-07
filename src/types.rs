use std::ffi::{c_char, c_void};

pub type Hachimi = *mut c_void;
pub type Interceptor = *mut c_void;
pub type Address = usize;
pub type String = *mut c_char;