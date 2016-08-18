// Wrappers around items provided by DXGI1_4.h and DXGI.lib
use declspec::*;
use dxgi1_3::*;
use unknwn::*;

use winapi;

use std;
use std::ops::Deref;

pub struct IDXGIFactory4 {
    ptr: *mut winapi::IDXGIFactory4,
}

impl IDXGIFactory4 {
}

impl From<*mut std::os::raw::c_void> for IDXGIFactory4 {
    fn from(source : *mut std::os::raw::c_void) -> Self {
        IDXGIFactory4 {
            ptr: source as *mut winapi::IDXGIFactory4,
        }
    }
}

impl Deref for IDXGIFactory4 {
    type Target=IDXGIFactory3;

    fn deref(&self) -> &IDXGIFactory3 {
        unsafe { &*(self as *const IDXGIFactory4 as *const IDXGIFactory3) }
    }
}

DEFINE_GUID! { IDXGIFACTORY4_GUID, 0x1bc6ea02, 0xef36, 0x464f, 0xbf, 0x0c, 0x21, 0xca, 0x39, 0xe5, 0x16, 0x8a }

impl DeclspecUUID for IDXGIFactory4 {
    fn uuid() -> winapi::GUID {
        IDXGIFACTORY4_GUID
    }
}