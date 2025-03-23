use std::{slice, string::FromUtf16Error};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Il2CppObject {
    pub klass: *const std::ffi::c_void,   // *const Il2CppClass
    pub monitor: *const std::ffi::c_void, // *const MonitorData
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Il2CppString {
    pub obj: Il2CppObject,
    pub m_stringLength: u32,
    pub m_firstChar: u16,
}

impl Il2CppString {
    pub fn to_string(&self) -> Result<String, FromUtf16Error> {
        unsafe {
            let ptr = &self.m_firstChar;
            let array = std::slice::from_raw_parts(ptr, self.m_stringLength as usize);
            String::from_utf16(&array)
        }
    }
}

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy)]
pub struct Il2CppArray<T> {
    pub obj: Il2CppObject,
    pub bounds: *const std::ffi::c_void, // *const Il2CppArrayBounds
    pub max_length: u32,
    // This is the first item of some pointer
    vector: *const T,
}

impl<T> Il2CppArray<T> {
    pub fn to_slice(&self) -> &[*const T] {
        unsafe {
            let ptr = &self.vector;
            slice::from_raw_parts(ptr, self.max_length as usize)
        }
    }
}
