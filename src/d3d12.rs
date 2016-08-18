// Wrappers around items provided by D3D12.h, D3D12.lib and D3D12.dll
extern crate d3d12;

use d3dcommon::*;
use declspec::*;
use unknwn::*;

use winapi;

use std;

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