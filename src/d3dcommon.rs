// Wrappers around items provided by D3DCommon.h, D3DCompiler.lib, and D3DCompiler_47.dll
extern crate dxguid;

use declspec::*;
use unknwn::*;

use winapi;

use std::ops::Deref;

win32_enum! {
    enum D3DFeatureLevel(winapi::D3D_FEATURE_LEVEL) {
        FeatureLevel91 = winapi::D3D_FEATURE_LEVEL_9_1,
        FeatureLevel92 = winapi::D3D_FEATURE_LEVEL_9_2,
        FeatureLevel93 = winapi::D3D_FEATURE_LEVEL_9_3,
        FeatureLevel100 = winapi::D3D_FEATURE_LEVEL_10_0,
        FeatureLevel101 = winapi::D3D_FEATURE_LEVEL_10_1,
        FeatureLevel110 = winapi::D3D_FEATURE_LEVEL_11_0,
        FeatureLevel111 = winapi::D3D_FEATURE_LEVEL_11_1,
        FeatureLevel120 = winapi::D3D_FEATURE_LEVEL_12_0,
        FeatureLevel121 = winapi::D3D_FEATURE_LEVEL_12_1,
    }
}

pub type ID3DBlob = ID3D10Blob;

#[derive(Debug)]
pub struct ID3D10Blob {
    ptr: *mut winapi::ID3DBlob,
}

impl ID3D10Blob {
}

impl From<*mut winapi::ID3D10Blob> for ID3D10Blob {
    fn from(source : *mut winapi::ID3D10Blob) -> Self {
        ID3D10Blob {
            ptr: source,
        }
    }
}

impl Deref for ID3D10Blob {
    type Target=IUnknown;

    fn deref(&self) -> &IUnknown {
        unsafe { &*(self as *const ID3D10Blob as *const IUnknown) }
    }
}

impl DeclspecUUID for ID3D10Blob {
    fn uuid() -> winapi::GUID {
        dxguid::IID_ID3D10Blob
    }
}