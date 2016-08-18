// Wrappers around items provided by DXGI1_2.h and DXGI.lib
use declspec::*;
use dxgi::*;
use unknwn::*;

use winapi;

use std::ops::Deref;

pub struct IDXGIFactory2 {
    ptr: *mut winapi::IDXGIFactory1,
}

impl IDXGIFactory2 {
}

impl Deref for IDXGIFactory2 {
    type Target=IDXGIFactory1;

    fn deref(&self) -> &IDXGIFactory1 {
        unsafe { &*(self as *const IDXGIFactory2 as *const IDXGIFactory1) }
    }
}

DEFINE_GUID! { IDXGIFACTORY2_GUID, 0x50c83a1c, 0xe072, 0x4c48, 0x87, 0xb0, 0x36, 0x30, 0xfa, 0x36, 0xa6, 0xd0 }

impl DeclspecUUID for IDXGIFactory2 {
    fn uuid() -> winapi::GUID {
        IDXGIFACTORY2_GUID
    }
}