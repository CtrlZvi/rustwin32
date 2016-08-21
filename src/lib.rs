#[macro_use(DEFINE_GUID)]
extern crate winapi;
#[macro_use(bitflags)]
extern crate bitflags;

pub use d3d12::*;
pub use d3d12sdklayers::*;
pub use d3dcommon::*;
pub use d3dx12::*;
pub use declspec::*;
pub use dxgi::*;
pub use dxgi1_2::*;
pub use dxgi1_3::*;
pub use dxgi1_4::*;
pub use dxgi1_5::*;
pub use dxgiformat::*;
pub use dxgitype::*;
pub use kernel32::*;
pub use unknwn::*;
pub use user32::*;
pub use windef::*;

#[macro_use]
mod macros;
mod d3d12;
mod d3d12sdklayers;
mod d3dcommon;
mod d3dx12;
mod declspec;
mod dxgi;
mod dxgi1_2;
mod dxgi1_3;
mod dxgi1_4;
mod dxgi1_5;
mod dxgiformat;
mod dxgitype;
mod kernel32;
mod unknwn;
mod user32;
mod windef;

#[allow(non_snake_case)]
pub mod D3D12CommandQueueFlags;
#[allow(non_snake_case)]
pub mod D3D12RootSignatureFlags;
#[allow(non_snake_case)]
pub mod DXGISwapChainFlags;
#[allow(non_snake_case)]
pub mod DXGIUsage;