// Wrappers around items provided by DXGI1_5.h and DXGI.lib
use declspec::*;
use dxgi1_4::*;

use winapi;

use std;
use std::ops::Deref;

pub struct IDXGIFactory5 {
    // TODO(zeffron 2016-08-20): Once dxgi1_5 is added to winapi, use that
    // instead
    ptr: *mut winapi_ext::IDXGIFactory5,
}

impl IDXGIFactory5 {
    // TODO(zeffron 2016-08-20): Figure out how to handle the type
    // appropriately (e.g. take in bool instead of u32)
    pub fn check_feature_support<T>(&self, feature: DXGIFeature) -> Result<T, std::io::Error> {
        let mut feature_support_data: T = unsafe { std::mem::uninitialized() };
        match unsafe {
            (*self.ptr).CheckFeatureSupport(
                feature.into(),
                &mut feature_support_data as *mut T as *mut std::os::raw::c_void,
                std::mem::size_of::<T>() as u32,
            )
        } {
            winapi::S_OK | winapi::S_FALSE => Ok(feature_support_data),
            // TODO(zeffron 2016-08-20): Figure out a better error type for
            // HRESULTS and switch to it
            result => panic!("{:x}", result),
        }
    }
}

impl From<*mut std::os::raw::c_void> for IDXGIFactory5 {
    fn from(source : *mut std::os::raw::c_void) -> Self {
        IDXGIFactory5 {
            // TODO(zeffron 2016-08-20): Once dxgi1_5 is added to winapi, use
            // that instead
            ptr: source as *mut winapi_ext::IDXGIFactory5,
        }
    }
}

impl Deref for IDXGIFactory5 {
    type Target=IDXGIFactory4;

    fn deref(&self) -> &IDXGIFactory4 {
        unsafe { &*(self as *const IDXGIFactory5 as *const IDXGIFactory4) }
    }
}

DEFINE_GUID! { IDXGIFACTORY5_GUID, 0x7632e1f5, 0xee65, 0x4dca, 0x87, 0xfd, 0x84, 0xcd, 0x75, 0xf8, 0x83, 0x8d }

impl DeclspecUUID for IDXGIFactory5 {
    fn uuid() -> winapi::GUID {
        IDXGIFACTORY5_GUID
    }
}

pub enum DXGIFeature {
    AllowTearing,
}

// TODO(zeffron 2016-08-20): Once dxgi1_5 is added to winapi, use that instead
impl From<DXGIFeature> for winapi_ext::DXGI_FEATURE {
    fn from(source: DXGIFeature) -> winapi_ext::DXGI_FEATURE {
        match source {
            DXGIFeature::AllowTearing => winapi_ext::DXGI_FEATURE_ALLOW_TEARING,
        }
    }
}

// TODO(zeffron 2016-08-20): Once dxgi1_5 is added to winapi, use that instead
mod winapi_ext {
    #[repr(C)] #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct DXGI_FEATURE(pub u32);
    pub const DXGI_FEATURE_ALLOW_TEARING: DXGI_FEATURE = DXGI_FEATURE(0);

    #[repr(C)] #[allow(missing_copy_implementations)] #[allow(non_snake_case)]
    pub struct IDXGIFactory5Vtbl {
        pub parent: ::winapi::IDXGIFactory4Vtbl,
        pub CheckFeatureSupport: unsafe extern "system" fn(
            This: *mut IDXGIFactory5,
            Feature: DXGI_FEATURE, pFeatureSupportData: *mut ::std::os::raw::c_void,
            FeatureSupportDataSize: ::winapi::UINT
        ) -> ::winapi::HRESULT
    }
    #[repr(C)] #[derive(Debug)] #[allow(missing_copy_implementations)] #[allow(non_snake_case)]
    pub struct IDXGIFactory5 {
        pub lpVtbl: *const IDXGIFactory5Vtbl
    }
    impl IDXGIFactory5 {
        #[inline]
        #[allow(non_snake_case)]
        pub unsafe fn CheckFeatureSupport(&mut self, Feature: DXGI_FEATURE, pFeatureSupportData: *mut ::std::os::raw::c_void, FeatureSupportDataSize: ::winapi::UINT) -> ::winapi::HRESULT {
            ((*self.lpVtbl).CheckFeatureSupport)(self, Feature, pFeatureSupportData, FeatureSupportDataSize)
        }
    }
    impl ::std::ops::Deref for IDXGIFactory5 {
        type Target = ::winapi::IDXGIFactory4;
        #[inline]
        fn deref(&self) -> &::winapi::IDXGIFactory4 {
            unsafe { ::std::mem::transmute(self) }
        }
    }
    impl ::std::ops::DerefMut for IDXGIFactory5 {
        #[inline]
        fn deref_mut(&mut self) -> &mut ::winapi::IDXGIFactory4 {
            unsafe { ::std::mem::transmute(self) }
        }
    }
}