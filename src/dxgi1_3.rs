// Wrappers around items provided by DXGI1_3.h and DXGI.lib
use declspec::*;
use dxgi1_2::*;
use unknwn::*;

use winapi;

use std::ops::Deref;

pub struct IDXGIFactory3 {
    ptr: *mut winapi::IDXGIFactory2,
}

impl IDXGIFactory3 {
}

impl Deref for IDXGIFactory3 {
    type Target=IDXGIFactory2;

    fn deref(&self) -> &IDXGIFactory2 {
        unsafe { &*(self as *const IDXGIFactory3 as *const IDXGIFactory2) }
    }
}

DEFINE_GUID! { IDXGIFACTORY3_GUID, 0x25483823, 0xcd46, 0x4c7d, 0x86, 0xca, 0x47, 0xaa, 0x95, 0xb8, 0x37, 0xbd }

impl DeclspecUUID for IDXGIFactory3 {
    fn uuid() -> winapi::GUID {
        IDXGIFACTORY3_GUID
    }
}