// Wrappers around items provided by Unknwn.h and Uknwn.idl
use declspec::*;

use winapi;

use std;

pub struct IUnknown {
    pub ptr: *mut winapi::IUnknown,
}

impl IUnknown {
    #[inline(always)]
    pub fn add_ref(&self) -> u32 {
        unsafe { (*self.ptr).AddRef() }
    }

    #[inline(always)]
    pub fn query_interface<T>(&self) -> Result<T, std::io::Error> where T: DeclspecUUID + From<*mut std::os::raw::c_void> {
        let mut interface: *mut std::os::raw::c_void = unsafe { std::mem::uninitialized() };
        let result = unsafe { (*self.ptr).QueryInterface(&T::uuid(), &mut interface) };
        match result {
            winapi::S_OK => Ok(interface.into()),
            // TODO(zeffron 2016-08-20): Figure out a better error type for
            // HRESULTS and switch to it
            winapi::E_NOINTERFACE => Err(std::io::Error::last_os_error()),
            winapi::E_POINTER => Err(std::io::Error::last_os_error()),
            result => panic!("{}", result),
        }
    }

    #[inline(always)]
    pub fn release(&self) -> u32 {
        unsafe { (*self.ptr).Release() }
    }

    pub unsafe fn as_ptr(&self) -> *mut winapi::IUnknown {
        self.ptr
    }
}

impl From<*mut std::os::raw::c_void> for IUnknown {
    fn from(source : *mut std::os::raw::c_void) -> Self {
        IUnknown {
            ptr: source as *mut winapi::IUnknown,
        }
    }
}

DEFINE_GUID! { IUNKNOWN_GUID, 0x00000000, 0x0000, 0x0000, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46 }

impl DeclspecUUID for IUnknown {
    fn uuid() -> winapi::GUID {
        IUNKNOWN_GUID
    }
}