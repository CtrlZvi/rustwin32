// Wrappers around items provided by DXGI.h, DXGI.lib, and Dxgi.dll
extern crate dxgi;

use declspec::*;
use unknwn::*;

use winapi;

use std;
use std::ops::Deref;
use std::os::windows::ffi::OsStringExt;

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
    #[inline]
    pub fn enum_adapters_1(&self, adapter_index: u32) -> Result<IDXGIAdapter1, std::io::Error> {
        let mut adapter: *mut winapi::IDXGIAdapter1 = unsafe { std::mem::uninitialized() };
        match unsafe { (*self.ptr).EnumAdapters1(adapter_index, &mut adapter as *mut *mut winapi::IDXGIAdapter1) } {
            winapi::S_OK => Ok(adapter.into()),
            _ => panic!("{:?}", std::io::Error::last_os_error()),
        }
    }
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

pub struct IDXGIAdapter {
    ptr: *mut winapi::IDXGIAdapter,
}

impl IDXGIAdapter {
}

impl Deref for IDXGIAdapter {
    type Target=IDXGIObject;

    fn deref(&self) -> &IDXGIObject {
        unsafe { &*(self as *const IDXGIAdapter as *const IDXGIObject) }
    }
}

DEFINE_GUID! { IDXGIADAPTER_GUID, 0x2411e7e1, 0x12ac, 0x4ccf, 0xbd, 0x14, 0x97, 0x98, 0xe8, 0x53, 0x4d, 0xc0 }

impl DeclspecUUID for IDXGIAdapter {
    fn uuid() -> winapi::GUID {
        IDXGIADAPTER_GUID
    }
}

pub struct IDXGIAdapter1 {
    ptr: *mut winapi::IDXGIAdapter1,
}

impl IDXGIAdapter1 {
    #[inline]
    pub fn get_description_1(&self) -> Result<DXGIAdapterDescription1, std::io::Error> {
        let mut description: winapi::DXGI_ADAPTER_DESC1 = unsafe { std::mem::uninitialized() };
        match unsafe { (*self.ptr).GetDesc1(&mut description as *mut winapi::DXGI_ADAPTER_DESC1) } {
            winapi::S_OK => Ok(description.into()),
            _ => panic!("{:?}", std::io::Error::last_os_error()),
        }
    }
}

impl From<*mut winapi::IDXGIAdapter1> for IDXGIAdapter1 {
    fn from(source : *mut winapi::IDXGIAdapter1) -> Self {
        IDXGIAdapter1 {
            ptr: source as *mut winapi::IDXGIAdapter1,
        }
    }
}

impl Deref for IDXGIAdapter1 {
    type Target=IDXGIAdapter;

    fn deref(&self) -> &IDXGIAdapter {
        unsafe { &*(self as *const IDXGIAdapter1 as *const IDXGIAdapter) }
    }
}

DEFINE_GUID! { IDXGIADAPTER1_GUID, 0x29038f61, 0x3839, 0x4626, 0x91, 0xfd, 0x08, 0x68, 0x79, 0x01, 0x1a, 0x05 }

impl DeclspecUUID for IDXGIAdapter1 {
    fn uuid() -> winapi::GUID {
        IDXGIADAPTER1_GUID
    }
}

bitflags!{
    pub flags DXGIAdapterFlag : u32 {
        const NONE = 0,
        const REMOTE = 1,
        const SOFTWARE = 2,
    }
}

pub struct DXGIAdapterDescription1 {
    pub description: String,
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_system_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    // TODO(zeffron 2016-08-17): Replace the winapi types in the parameters
    pub adapter_luid: winapi::LUID,
    pub flags: DXGIAdapterFlag,
}

impl From<winapi::DXGI_ADAPTER_DESC1> for DXGIAdapterDescription1 {
    fn from(source: winapi::DXGI_ADAPTER_DESC1) -> Self {
        DXGIAdapterDescription1 {
            // TODO(zeffron 2016-08-17): Handle the case where the string could not be converted
            description: std::ffi::OsString::from_wide(source.Description.iter().take_while(|c| **c != 0).map(|c| *c).collect::<Vec<u16>>().as_slice()).into_string().unwrap(),
            vendor_id: source.VendorId,
            device_id: source.DeviceId,
            sub_system_id: source.SubSysId,
            revision: source.Revision,
            dedicated_video_memory: source.DedicatedVideoMemory as usize,
            dedicated_system_memory: source.DedicatedSystemMemory as usize,
            shared_system_memory: source.SharedSystemMemory as usize,
            adapter_luid: source.AdapterLuid,
            // TODO(zeffron 2016-08-17): Handle the case where the flags are invalid
            flags: DXGIAdapterFlag::from_bits(source.Flags).unwrap(),
        }
    }
}

pub enum DXGISwapEffect {
    Discard,
    Sequential,
    FlipSequential,
    FlipDiscard,
}

impl From<DXGISwapEffect> for winapi::DXGI_SWAP_EFFECT {
    fn from(source: DXGISwapEffect) -> winapi::DXGI_SWAP_EFFECT {
        match source {
            DXGISwapEffect::Discard => winapi::DXGI_SWAP_EFFECT_DISCARD,
            DXGISwapEffect::Sequential => winapi::DXGI_SWAP_EFFECT_SEQUENTIAL,
            DXGISwapEffect::FlipSequential => winapi::DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
            // TODO(zeffron 2016-08-19): Switch to
            // winapi::DXGI_SWAP_EFFECT_FLIP_DISCARD once it exists
            DXGISwapEffect::FlipDiscard => winapi::DXGI_SWAP_EFFECT(4),
        }
    }
}