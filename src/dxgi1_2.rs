// Wrappers around items provided by DXGI1_2.h and DXGI.lib
extern crate dxguid;

use declspec::*;
use dxgi::*;
use dxgiformat::*;
use dxgitype::*;
use unknwn::*;
use user32::*;

use DXGISwapChainFlags;
use DXGIUsage;

use winapi;

use std;
use std::ops::Deref;

pub struct IDXGIFactory2 {
    ptr: *mut winapi::IDXGIFactory2,
}

impl IDXGIFactory2 {
    pub fn create_swap_chain_for_window(&self, device: &IUnknown, window: Window, description: DXGISwapChainDescription1, fullscreen_description: Option<DXGISwapChainFullscreenDescription>, restrict_to_output: Option<&IDXGIOutput>) -> Result<IDXGISwapChain1, std::io::Error> {
        let mut swap_chain: *mut winapi::IDXGISwapChain1 = unsafe { std::mem::uninitialized() };
        match unsafe {
            (*self.ptr).CreateSwapChainForHwnd(
                device.ptr,
                window.handle,
                &description.into(),
                match fullscreen_description {
                    Some(fullscreen_description) => &fullscreen_description.into(),
                    None => std::ptr::null_mut(),
                },
                match restrict_to_output {
                    Some(output) => output.ptr as *mut winapi::IDXGIOutput,
                    None => std::ptr::null_mut(),
                },
                &mut swap_chain,
            )
        } {
            winapi::S_OK => Ok(swap_chain.into()),
            result => panic!("{:x}", result),
        }
    }
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

impl From<DXGISwapChainDescription1> for winapi::DXGI_SWAP_CHAIN_DESC1 {
    fn from(source: DXGISwapChainDescription1) -> Self {
        winapi::DXGI_SWAP_CHAIN_DESC1 {
            Width: source.width,
            Height: source.height,
            Format: source.format.into(),
            Stereo: source.stereo as i32,
            SampleDesc: source.sample_description.into(),
            BufferUsage: winapi::DXGI_USAGE(source.buffer_usage.bits()),
            BufferCount: source.buffer_count,
            Scaling: source.scaling.into(),
            SwapEffect: source.swap_effect.into(),
            AlphaMode: source.alpha_mode.into(),
            Flags: source.flags.bits(),
        }
    }
}

win32_enum! {
    enum DXGIScaling(winapi::DXGI_SCALING) {
        Stretch = winapi::DXGI_SCALING_STRETCH,
        None = winapi::DXGI_SCALING_NONE,
        AspectRatioStretch = winapi::DXGI_SCALING_ASPECT_RATIO_STRETCH,
    }
}

win32_enum! {
    enum DXGIAlphaMode(winapi::DXGI_ALPHA_MODE) {
        Unspecified = winapi::DXGI_ALPHA_MODE_UNSPECIFIED,
        Premultiplied = winapi::DXGI_ALPHA_MODE_PREMULTIPLIED,
        Straight = winapi::DXGI_ALPHA_MODE_STRAIGHT,
        Ignore = winapi::DXGI_ALPHA_MODE_IGNORE,
    }
}

pub struct DXGISwapChainFullscreenDescription {
    refresh_rate: DXGIRational,
    scanline_ordering: DXGIModeScanlineOrder,
    scaling: DXGIModeScaling,
    windowed: bool,
}

impl From<DXGISwapChainFullscreenDescription> for winapi::DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn from(source: DXGISwapChainFullscreenDescription) -> Self {
        winapi::DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
            RefreshRate: source.refresh_rate.into(),
            ScanlineOrdering: source.scanline_ordering.into(),
            Scaling: source.scaling.into(),
            Windowed: source.windowed as i32,
        }
    }
}

pub struct DXGIRational {
    numerator: u32,
    denominator: u32,
}

impl From<DXGIRational> for winapi::DXGI_RATIONAL {
    fn from(source: DXGIRational) -> Self {
        winapi::DXGI_RATIONAL {
            Numerator: source.numerator,
            Denominator: source.denominator,
        }
    }
}

win32_enum! {
    enum DXGIModeScanlineOrder(winapi::DXGI_MODE_SCANLINE_ORDER) {
        Unspecified = winapi::DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
        Progressive = winapi::DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE,
        UpperFieldFirst = winapi::DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST,
        LowerFieldFirst = winapi::DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST,
    }
}

win32_enum! {
    enum DXGIModeScaling(winapi::DXGI_MODE_SCALING) {
        Unspecified = winapi::DXGI_MODE_SCALING_UNSPECIFIED,
        Centered = winapi::DXGI_MODE_SCALING_CENTERED,
        Stretched = winapi::DXGI_MODE_SCALING_STRETCHED,
    }
}

pub struct IDXGISwapChain1 {
    ptr: *mut winapi::IDXGISwapChain1,
}

impl IDXGISwapChain1 {
}

impl Deref for IDXGISwapChain1 {
    type Target=IDXGISwapChain;

    fn deref(&self) -> &IDXGISwapChain {
        unsafe { &*(self as *const IDXGISwapChain1 as *const IDXGISwapChain) }
    }
}

impl From<*mut winapi::IDXGISwapChain1> for IDXGISwapChain1 {
    fn from(source: *mut winapi::IDXGISwapChain1) -> Self {
        IDXGISwapChain1 {
            ptr: source as *mut winapi::IDXGISwapChain1,
        }
    }
}

impl DeclspecUUID for IDXGISwapChain1 {
    fn uuid() -> winapi::GUID {
        dxguid::IID_IDXGISwapChain1
    }
}