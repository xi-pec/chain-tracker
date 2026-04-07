use std::ffi::{c_void, c_char};

pub type IString = *mut c_char;
pub type IImage = *mut c_void;
pub type IClass = *mut c_void;
pub type IMethod = *mut c_void;
pub type ITypeEnum = u32;
pub type IField = *mut c_void;
pub type IObject = *mut c_void;
pub type IThread = *mut c_void;
pub type IArray = *mut c_void;

#[repr(C)]
pub struct IObjectP {
    pub data: IObjectDataP,
}

#[repr(C)]
pub struct IObjectDataP {
    pub class: IClass
}

#[repr(C)]
pub struct IClassP {
    pad1: [u8; 64],
    pub element_class: IClass,

    _pad2: [u8; 188],
    pub element_size: u32
}

#[repr(C)]
pub struct IArrayP {
    _pad: [u8; 24],
    pub max_length: usize
}