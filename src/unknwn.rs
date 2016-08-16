// Wrappers around items provided by Unknwn.h and Uknwn.idl
use declspec::*;

use winapi;

use std;

pub struct IUnknown {
    ptr: *mut winapi::IUnknown,
}

impl IUnknown {
    #[inline(always)]
    pub fn add_ref(&self) -> u32 {
        unsafe { (*self.ptr).AddRef() }
    }

    // TODO(zeffron 2016-08-15): Replace the winapi types in the parameters
    #[inline(always)]
    pub fn query_interface(&self, riid: winapi::REFIID, object: *mut *mut winapi::c_void) -> winapi::HRESULT {
        unsafe { (*self.ptr).QueryInterface(riid, object) }
    }

    #[inline(always)]
    pub fn release(&self) -> u32 {
        unsafe {(*self.ptr).Release() }
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