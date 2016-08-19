// Wrappers around items provided by D3D12.h, D3D12.lib and D3D12.dll
extern crate d3d12;

use d3dcommon::*;
use declspec::*;
use unknwn::*;

use D3D12CommandQueueFlags;

use winapi;

use std;
use std::ops::Deref;

pub fn d3d12_get_debug_interface<T : DeclspecUUID + From<*mut std::os::raw::c_void>>() -> Result<T, std::io::Error> {
    let mut debug : *mut std::os::raw::c_void = unsafe { std::mem::uninitialized() };
    match unsafe { d3d12::D3D12GetDebugInterface(&T::uuid(), &mut debug as *mut *mut std::os::raw::c_void) } {
         winapi::S_OK => Ok(debug.into()),
         winapi::S_FALSE => { println!("Received the alternate success"); Ok(debug.into()) },
         _ => panic!("{:?}", std::io::Error::last_os_error()),
     }
}

pub fn d3d12_create_device<T: DeclspecUUID + From<*mut std::os::raw::c_void>>(adapter: Option<&IUnknown>, minimum_feature_level: D3DFeatureLevel) -> Result<T, std::io::Error> {
    let mut device: *mut std::os::raw::c_void = unsafe { std::mem::uninitialized() };
    match unsafe {
        d3d12::D3D12CreateDevice(
            match adapter {
                Some(adapter) => adapter.as_ptr(),
                None => std::ptr::null_mut(),
            },
            match minimum_feature_level {
                D3DFeatureLevel::FeatureLevel91 => winapi::D3D_FEATURE_LEVEL_9_1,
                D3DFeatureLevel::FeatureLevel92 => winapi::D3D_FEATURE_LEVEL_9_2,
                D3DFeatureLevel::FeatureLevel93 => winapi::D3D_FEATURE_LEVEL_9_3,
                D3DFeatureLevel::FeatureLevel100 => winapi::D3D_FEATURE_LEVEL_10_0,
                D3DFeatureLevel::FeatureLevel101 => winapi::D3D_FEATURE_LEVEL_10_1,
                D3DFeatureLevel::FeatureLevel110 => winapi::D3D_FEATURE_LEVEL_11_0,
                D3DFeatureLevel::FeatureLevel111 => winapi::D3D_FEATURE_LEVEL_11_1,
                D3DFeatureLevel::FeatureLevel120 => winapi::D3D_FEATURE_LEVEL_12_0,
                D3DFeatureLevel::FeatureLevel121 => winapi::D3D_FEATURE_LEVEL_12_1,
            },
            &T::uuid(),
            &mut device as *mut *mut std::os::raw::c_void
        )
    } {
         winapi::S_OK | winapi::S_FALSE => Ok(device.into()),
         // TODO(zeffron 2016-08-18): Implement an appropriate error type and
         // switch to it
         result => panic!("{:x}", result),
     }
}

pub struct ID3D12Object {
    ptr: *mut winapi::ID3D12Object,
}

impl ID3D12Object {
}

impl Deref for ID3D12Object {
    type Target=IUnknown;

    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const ID3D12Object as *const IUnknown) }
    }
}

DEFINE_GUID! { ID3D12OBJECT_GUID, 0xc4fec28f, 0x7966, 0x4e95, 0x9f, 0x94, 0xf4, 0x31, 0xcb, 0x56, 0xc3, 0xb8 }

impl DeclspecUUID for ID3D12Object {
    fn uuid() -> winapi::GUID {
        ID3D12OBJECT_GUID
    }
}

pub struct ID3D12Device {
    ptr: *mut winapi::ID3D12Device,
}

impl ID3D12Device {
    pub fn create_command_queue<T>(&self, description: &D3D12CommandQueueDescription) -> Result<T, std::io::Error> where T: DeclspecUUID + From<*mut std::os::raw::c_void> {
        let mut command_queue: *mut std::os::raw::c_void = unsafe { std::mem::uninitialized() };
        match unsafe { (*self.ptr).CreateCommandQueue(
            &winapi::D3D12_COMMAND_QUEUE_DESC {
                Type: match description.list_type {
                    D3D12CommandListType::Direct => winapi::D3D12_COMMAND_LIST_TYPE_DIRECT,
                    D3D12CommandListType::Bundle => winapi::D3D12_COMMAND_LIST_TYPE_BUNDLE,
                    D3D12CommandListType::Compute => winapi::D3D12_COMMAND_LIST_TYPE_COMPUTE,
                    D3D12CommandListType::Copy => winapi::D3D12_COMMAND_LIST_TYPE_COPY,
                },
                Priority: description.priority,
                Flags: winapi::D3D12_COMMAND_QUEUE_FLAGS(description.flags.bits()),
                NodeMask: description.node_mask,
            },
            &T::uuid(),
            &mut command_queue
        ) } {
            winapi::S_OK | winapi::S_FALSE => Ok(command_queue.into()),
            // TODO(zeffron 2016-08-18): Implement an appropriate error type and
            // switch to it
            result => panic!("{:x}", result),
        }
    }
}

impl Deref for ID3D12Device {
    type Target=ID3D12Object;

    fn deref(&self) -> &ID3D12Object {
        unsafe { &*(self as *const ID3D12Device as *const ID3D12Object) }
    }
}

impl From<*mut std::os::raw::c_void> for ID3D12Device {
    fn from(source: *mut std::os::raw::c_void) -> Self {
        ID3D12Device {
            ptr: source as *mut winapi::ID3D12Device,
        }
    }
}

DEFINE_GUID! { ID3D12DEVICE_GUID, 0x189819f1, 0x1db6, 0x4b57, 0xbe, 0x54, 0x18, 0x21, 0x33, 0x9b, 0x85, 0xf7 }

impl DeclspecUUID for ID3D12Device {
    fn uuid() -> winapi::GUID {
        ID3D12DEVICE_GUID
    }
}

pub struct D3D12CommandQueueDescription {
    pub list_type: D3D12CommandListType,
    pub priority: i32,
    pub flags: D3D12CommandQueueFlags::Flags,
    pub node_mask: u32,
}

pub enum D3D12CommandListType {
    Direct,
    Bundle,
    Compute,
    Copy,
}

pub struct ID3D12CommandQueue {
    ptr: *mut winapi::ID3D12CommandQueue,
}

impl ID3D12CommandQueue {
}

impl From<*mut std::os::raw::c_void> for ID3D12CommandQueue {
    fn from(source: *mut std::os::raw::c_void) -> Self {
        ID3D12CommandQueue {
            ptr: source as *mut winapi::ID3D12CommandQueue,
        }
    }
}

impl Deref for ID3D12CommandQueue {
    type Target=ID3D12Pageable;

    fn deref(&self) -> &ID3D12Pageable {
        unsafe { &*(self as *const ID3D12CommandQueue as *const ID3D12Pageable) }
    }
}

DEFINE_GUID! { ID3D12COMMANDQUEUE_GUID, 0x0ec870a6, 0x5d7e, 0x4c22, 0x8c, 0xfc, 0x5b, 0xaa, 0xe0, 0x76, 0x16, 0xed }

impl DeclspecUUID for ID3D12CommandQueue {
    fn uuid() -> winapi::GUID {
        ID3D12COMMANDQUEUE_GUID
    }
}

pub struct ID3D12Pageable {
    ptr: *mut winapi::ID3D12Pageable,
}

impl ID3D12Pageable {
}

impl Deref for ID3D12Pageable {
    type Target=ID3D12DeviceChild;

    fn deref(&self) -> &ID3D12DeviceChild {
        unsafe { &*(self as *const ID3D12Pageable as *const ID3D12DeviceChild) }
    }
}

DEFINE_GUID! { ID3D12PAGEABLE, 0x63ee58fb, 0x1268, 0x4835, 0x86, 0xda, 0xf0, 0x08, 0xce, 0x62, 0xf0, 0xd6 }

impl DeclspecUUID for ID3D12Pageable {
    fn uuid() -> winapi::GUID {
        ID3D12PAGEABLE
    }
}

pub struct ID3D12DeviceChild {
    ptr: *mut winapi::ID3D12DeviceChild,
}

impl ID3D12DeviceChild {
}

impl Deref for ID3D12DeviceChild {
    type Target=ID3D12Object;

    fn deref(&self) -> &ID3D12Object {
        unsafe { &*(self as *const ID3D12DeviceChild as *const ID3D12Object) }
    }
}

DEFINE_GUID! { ID3D12DEVICECHILD_GUID, 0x905db94b, 0xa00c, 0x4140, 0x9d, 0xf5, 0x2b, 0x64, 0xca, 0x9e, 0xa3, 0x57 }

impl DeclspecUUID for ID3D12DeviceChild {
    fn uuid() -> winapi::GUID {
        ID3D12DEVICECHILD_GUID
    }
}
