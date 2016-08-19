// Wrappers around items provided by DXGI1_2.h and DXGI.lib
use declspec::*;
use dxgi::*;
use dxgiformat::*;
use dxgitype::*;
use unknwn::*;

use DXGISwapChainFlags;
use DXGIUsage;

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

pub struct DXGISwapChainDescription1 {
    pub width: u32,
    pub height: u32,
    pub format: DXGIFormat,
    pub stereo: bool,
    pub sample_description: DXGISampleDescription,
    pub buffer_usage: DXGIUsage::Flags,
    pub buffer_count: u32,
    pub scaling: DXGIScaling,
    pub swap_effect: DXGISwapEffect,
    pub alpha_mode: DXGIAlphaMode,
    pub flags: DXGISwapChainFlags::Flags,
}

pub enum DXGIScaling {
    Stretch,
    None,
    AspectRatioStretch,
}

impl From<DXGIScaling> for winapi::DXGI_SCALING {
    fn from(source: DXGIScaling) -> winapi::DXGI_SCALING {
        match source {
            DXGIScaling::Stretch => winapi::DXGI_SCALING_STRETCH,
            DXGIScaling::None => winapi::DXGI_SCALING_NONE,
            DXGIScaling::AspectRatioStretch => winapi::DXGI_SCALING_ASPECT_RATIO_STRETCH,
        }
    }
}

pub enum DXGIAlphaMode {
    Unspecified,
    Premultiplied,
    Straight,
    Ignore,
}

impl From<DXGIAlphaMode> for winapi::DXGI_ALPHA_MODE {
    fn from(source: DXGIAlphaMode) -> winapi::DXGI_ALPHA_MODE {
        match source {
            DXGIAlphaMode::Unspecified => winapi::DXGI_ALPHA_MODE_UNSPECIFIED,
            DXGIAlphaMode::Premultiplied => winapi::DXGI_ALPHA_MODE_PREMULTIPLIED,
            DXGIAlphaMode::Straight => winapi::DXGI_ALPHA_MODE_STRAIGHT,
            DXGIAlphaMode::Ignore => winapi::DXGI_ALPHA_MODE_IGNORE,
        }
    }
}
