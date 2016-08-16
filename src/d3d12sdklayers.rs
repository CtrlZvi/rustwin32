// Wrappers around items provided by d3d12sdklayers.h
use declspec::*;
use unknwn::*;

use winapi;

use std;
use std::ops::Deref;

pub struct ID3D12Debug {
    ptr: *mut winapi::ID3D12Debug,
}

impl ID3D12Debug {
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

    #[inline(always)]
    pub fn enable_debug_layer(&self) -> () {
        unsafe {(*self.ptr).EnableDebugLayer() }
    }
}

impl From<*mut std::os::raw::c_void> for ID3D12Debug {
    fn from(source : *mut std::os::raw::c_void) -> Self {
        ID3D12Debug {
            ptr: source as *mut winapi::ID3D12Debug,
        }
    }
}

impl Deref for ID3D12Debug {
    type Target = IUnknown;

    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const ID3D12Debug as *const IUnknown) }
    }
}

DEFINE_GUID! { ID3D12DEBUG_GUID, 0x344488b7, 0x6846, 0x474b, 0xb9, 0x89, 0xf0, 0x27, 0x44, 0x82, 0x45, 0xe0 }

impl DeclspecUUID for ID3D12Debug {
    fn uuid() -> winapi::GUID {
        ID3D12DEBUG_GUID
    }
}