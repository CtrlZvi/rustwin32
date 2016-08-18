// Wrappers around items provided by DXGI.h, DXGI.lib, and Dxgi.dll
extern crate dxgi;

use declspec::*;
use unknwn::*;

use winapi;

use std;
use std::ops::Deref;

pub struct IDXGIObject {
    ptr: *mut winapi::IDXGIObject,
}

impl IDXGIObject {
}

impl Deref for IDXGIObject {
    type Target=IUnknown;

    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const IDXGIObject as *const IUnknown) }
    }
}

DEFINE_GUID! { IDXGIOBJECT_GUID, 0xaec22fb8, 0x76f3, 0x4639, 0x9b, 0xe0, 0x28, 0xeb, 0x43, 0xa6, 0x7a, 0x2e }

impl DeclspecUUID for IDXGIObject {
    fn uuid() -> winapi::GUID {
        IDXGIOBJECT_GUID
    }
}

pub struct IDXGIFactory {
    ptr: *mut winapi::IDXGIFactory,
}

impl IDXGIFactory {
}

impl Deref for IDXGIFactory {
    type Target=IDXGIObject;

    fn deref(&self) -> &IDXGIObject {
        unsafe { &*(self as *const IDXGIFactory as *const IDXGIObject) }
    }
}

DEFINE_GUID! { IDXGIFACTORY_GUID, 0x7b7166ec, 0x21c7, 0x44ae, 0xb2, 0x1a, 0xc9, 0xae, 0x32, 0x1a, 0xe3, 0x69 }

impl DeclspecUUID for IDXGIFactory {
    fn uuid() -> winapi::GUID {
        IDXGIFACTORY_GUID
    }
}

pub struct IDXGIFactory1 {
    ptr: *mut winapi::IDXGIFactory1,
}

impl IDXGIFactory1 {
}

impl Deref for IDXGIFactory1 {
    type Target=IDXGIFactory;

    fn deref(&self) -> &IDXGIFactory {
        unsafe { &*(self as *const IDXGIFactory1 as *const IDXGIFactory) }
    }
}

DEFINE_GUID! { IDXGIFACTORY1_GUID, 0x770aae78, 0xf26f, 0x4dba, 0xa8, 0x29, 0x25, 0x3c, 0x83, 0xd1, 0xb3, 0x87 }

impl DeclspecUUID for IDXGIFactory1 {
    fn uuid() -> winapi::GUID {
        IDXGIFACTORY1_GUID
    }
}

pub fn create_dxgifactory1<T : DeclspecUUID + From<*mut std::os::raw::c_void>>() -> Result<T, std::io::Error> {
    let mut factory: *mut std::os::raw::c_void = unsafe { std::mem::uninitialized() };
    match unsafe { dxgi::CreateDXGIFactory1(&T::uuid(), &mut factory as *mut *mut std::os::raw::c_void) } {
         winapi::S_OK => Ok(factory.into()),
         _ => panic!("{:?}", std::io::Error::last_os_error()),
     }
}