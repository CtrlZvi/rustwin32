#[macro_use(DEFINE_GUID)]
extern crate winapi;
#[macro_use(bitflags)]
extern crate bitflags;

pub use d3d12::*;
pub use d3d12sdklayers::*;
pub use d3dcommon::*;
pub use declspec::*;
pub use dxgi::*;
pub use dxgi1_2::*;
pub use dxgi1_3::*;
pub use dxgi1_4::*;
pub use unknwn::*;
pub use user32::*;
pub use windef::*;

mod d3d12;
mod d3d12sdklayers;
mod d3dcommon;
mod declspec;
mod dxgi;
mod dxgi1_2;
mod dxgi1_3;
mod dxgi1_4;
mod unknwn;
mod user32;
mod windef;

#[allow(non_snake_case)]
pub mod D3D12CommandQueueFlags;