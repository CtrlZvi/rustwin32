// Wrappers around items provided by D3D12.h, D3D12.lib and D3D12.dll
extern crate d3d12;
extern crate dxguid;

use d3dcommon::*;
use declspec::*;
use unknwn::*;

use D3D12CommandQueueFlags;
use D3D12RootSignatureFlags;

use winapi;

use std;
use std::ops::{Deref, DerefMut};
use std::os::windows::ffi::OsStrExt;

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
    pub fn set_name(&mut self, name: &str) -> Result<(), std::io::Error> {
        match unsafe {
            (*self.ptr).SetName(
                // FIXME(zeffron 2016-08-21): I think the lifetime here is
                // incorrect. I don't think the vector we collect into survives
                // long enough.
                std::ffi::OsStr::new(name).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>().as_ptr(),
            )
        } {
            winapi::S_OK | winapi::S_FALSE => Ok(()),
            // TODO(zeffron 2016-08-18): Implement an appropriate error type and
            // switch to it
            result => panic!("{:x}", result),
        }
    }
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

    pub fn create_root_signature<T>(&self, node_mask: u32, blob_with_root_signature: &(), blob_length_in_bytes: usize) -> Result<T, std::io::Error> where T: DeclspecUUID + From<*mut std::os::raw::c_void> {
        let mut root_signature: *mut std::os::raw::c_void = unsafe { std::mem::uninitialized() };
        match unsafe { (*self.ptr).CreateRootSignature(node_mask, blob_with_root_signature as *const () as *const std::os::raw::c_void, blob_length_in_bytes as u64, &T::uuid(), &mut root_signature as *mut *mut std::os::raw::c_void) } {
            winapi::S_OK | winapi::S_FALSE => Ok(root_signature.into()),
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

win32_enum! {
    enum D3D12CommandListType(winapi::D3D12_COMMAND_LIST_TYPE) {
        Direct = winapi::D3D12_COMMAND_LIST_TYPE_DIRECT,
        Bundle = winapi::D3D12_COMMAND_LIST_TYPE_BUNDLE,
        Compute = winapi::D3D12_COMMAND_LIST_TYPE_COMPUTE,
        Copy = winapi::D3D12_COMMAND_LIST_TYPE_COPY,
    }
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

impl DerefMut for ID3D12CommandQueue {
    fn deref_mut(&mut self) -> &mut ID3D12Pageable {
        unsafe { &mut*(self as *mut ID3D12CommandQueue as *mut ID3D12Pageable) }
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

impl DerefMut for ID3D12Pageable {
    fn deref_mut(&mut self) -> &mut ID3D12DeviceChild {
        unsafe { &mut*(self as *mut ID3D12Pageable as *mut ID3D12DeviceChild) }
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

impl DerefMut for ID3D12DeviceChild {
    fn deref_mut(&mut self) -> &mut ID3D12Object {
        unsafe { &mut*(self as *mut ID3D12DeviceChild as *mut ID3D12Object) }
    }
}

DEFINE_GUID! { ID3D12DEVICECHILD_GUID, 0x905db94b, 0xa00c, 0x4140, 0x9d, 0xf5, 0x2b, 0x64, 0xca, 0x9e, 0xa3, 0x57 }

impl DeclspecUUID for ID3D12DeviceChild {
    fn uuid() -> winapi::GUID {
        ID3D12DEVICECHILD_GUID
    }
}

pub struct D3D12DescriptorRange {
    pub range_type: D3D12DescriptorRangeType,
    pub num_descriptors: u32,
    pub base_shader_register: u32,
    pub register_space: u32,
    pub offset_in_descriptors_from_table_start: u32,
}

impl<'a> From<&'a D3D12DescriptorRange> for winapi::D3D12_DESCRIPTOR_RANGE {
    fn from(source: &'a D3D12DescriptorRange) -> winapi::D3D12_DESCRIPTOR_RANGE {
        winapi::D3D12_DESCRIPTOR_RANGE {
            RangeType: source.range_type.into(),
            NumDescriptors: source.num_descriptors,
            BaseShaderRegister: source.base_shader_register,
            RegisterSpace: source.register_space,
            OffsetInDescriptorsFromTableStart: source.offset_in_descriptors_from_table_start,
        }
    }
}

win32_enum! {
    enum D3D12DescriptorRangeType(winapi::D3D12_DESCRIPTOR_RANGE_TYPE) {
        SRV = winapi::D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
        UAV = winapi::D3D12_DESCRIPTOR_RANGE_TYPE_UAV,
        CBV = winapi::D3D12_DESCRIPTOR_RANGE_TYPE_CBV,
    }
}

pub const D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND: u32 = winapi::D3D12_DESCRIPTOR_RANGE_OFFSET_APPEND;

pub enum D3D12RootParameterData<'a> {
    DescriptorTable(D3D12RootDescriptorTable<'a>),
    Constants(D3D12RootConstants),
    Descriptor(D3D12RootDescriptor),
}

impl<'a, 'b> From<&'a D3D12RootParameterData<'a>> for winapi::D3D12_ROOT_DESCRIPTOR_TABLE where winapi::D3D12_ROOT_DESCRIPTOR_TABLE: 'b, 'b: 'a {
    fn from(source: &'a D3D12RootParameterData<'a>) -> winapi::D3D12_ROOT_DESCRIPTOR_TABLE where winapi::D3D12_ROOT_DESCRIPTOR_TABLE: 'b, 'b: 'a {
        match *source {
            D3D12RootParameterData::DescriptorTable(ref descriptor_table) => descriptor_table.into(),
            D3D12RootParameterData::Constants(ref constants) => unsafe { *std::mem::transmute::<&winapi::D3D12_ROOT_CONSTANTS, &winapi::D3D12_ROOT_DESCRIPTOR_TABLE>(&constants.into()) },
            D3D12RootParameterData::Descriptor(ref descriptor) => unsafe { *std::mem::transmute::<&winapi::D3D12_ROOT_DESCRIPTOR, &winapi::D3D12_ROOT_DESCRIPTOR_TABLE>(&descriptor.into()) },
        }
    }
}

pub struct D3D12RootParameter<'a> {
    pub parameter_type: D3D12RootParameterType,
    pub union: D3D12RootParameterData<'a>,
    pub shader_visibility: D3D12ShaderVisibility,
}

impl<'a> From<&'a D3D12RootParameter<'a>> for winapi::D3D12_ROOT_PARAMETER where winapi::D3D12_ROOT_PARAMETER: 'a {
    fn from(source: &'a D3D12RootParameter<'a>) -> winapi::D3D12_ROOT_PARAMETER where winapi::D3D12_ROOT_PARAMETER: 'a {
        winapi::D3D12_ROOT_PARAMETER {
            ParameterType: source.parameter_type.into(),
            u: (&source.union).into(),
            ShaderVisibility: source.shader_visibility.into(),
        }
    }
}

win32_enum! {
    enum D3D12RootParameterType(winapi::D3D12_ROOT_PARAMETER_TYPE) {
        DescriptorTable = winapi::D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
        Constants32Bit = winapi::D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
        CBV = winapi::D3D12_ROOT_PARAMETER_TYPE_CBV,
        SRV= winapi::D3D12_ROOT_PARAMETER_TYPE_SRV,
        UAV = winapi::D3D12_ROOT_PARAMETER_TYPE_UAV,
    }
}

pub struct D3D12RootDescriptorTable<'a> {
    pub ranges: &'a[D3D12DescriptorRange],
}

impl<'a> From<&'a D3D12RootDescriptorTable<'a>> for winapi::D3D12_ROOT_DESCRIPTOR_TABLE where winapi::D3D12_ROOT_DESCRIPTOR_TABLE: 'a {
    fn from(source: &'a D3D12RootDescriptorTable<'a>) -> winapi::D3D12_ROOT_DESCRIPTOR_TABLE where winapi::D3D12_ROOT_DESCRIPTOR_TABLE: 'a {
        winapi::D3D12_ROOT_DESCRIPTOR_TABLE {
            NumDescriptorRanges: source.ranges.len() as u32,
            // FIXME(zeffron 2016-08-21): I think the lifetime here is
            // incorrect. I don't think the vector we collect into survives
            // long enough.
            pDescriptorRanges: source.ranges.iter().map(|range| range.into()).collect::<Vec<winapi::D3D12_DESCRIPTOR_RANGE>>().as_ptr(),
        }
    }
}

pub struct D3D12RootConstants {
    pub shader_register: u32,
    pub register_space: u32,
    pub num_32bit_values: u32,
}

impl<'a> From<&'a D3D12RootConstants> for winapi::D3D12_ROOT_CONSTANTS {
    fn from(source: &'a D3D12RootConstants) -> winapi::D3D12_ROOT_CONSTANTS {
        winapi::D3D12_ROOT_CONSTANTS {
            ShaderRegister: source.shader_register,
            RegisterSpace: source.register_space,
            Num32BitValues: source.num_32bit_values,
        }
    }
}

pub struct D3D12RootDescriptor {
    pub shader_register: u32,
    pub register_space: u32,
}

impl<'a> From<&'a D3D12RootDescriptor> for winapi::D3D12_ROOT_DESCRIPTOR {
    fn from(source: &'a D3D12RootDescriptor) -> winapi::D3D12_ROOT_DESCRIPTOR {
        winapi::D3D12_ROOT_DESCRIPTOR {
            ShaderRegister: source.shader_register,
            RegisterSpace: source.register_space,
        }
    }
}

win32_enum! {
    enum D3D12ShaderVisibility(winapi::D3D12_SHADER_VISIBILITY) {
        All = winapi::D3D12_SHADER_VISIBILITY_ALL,
        Vertex = winapi::D3D12_SHADER_VISIBILITY_VERTEX,
        Hull = winapi::D3D12_SHADER_VISIBILITY_HULL,
        Domain = winapi::D3D12_SHADER_VISIBILITY_DOMAIN,
        Geometry = winapi::D3D12_SHADER_VISIBILITY_GEOMETRY,
        Pixel = winapi::D3D12_SHADER_VISIBILITY_PIXEL,
    }
}

pub struct D3D12RootSignatureDescription<'a> {
    pub parameters: &'a[D3D12RootParameter<'a>],
    win32_parameters: Option<Vec<winapi::D3D12_ROOT_PARAMETER>>,
    pub static_samplers: Option<&'a[D3D12StaticSamplerDescription]>,
    win32_static_samplers: Option<Vec<winapi::D3D12_STATIC_SAMPLER_DESC>>,
    pub flags: D3D12RootSignatureFlags::Flags,
}

impl<'a> D3D12RootSignatureDescription<'a> {
    pub fn new(parameters: &'a[D3D12RootParameter<'a>], static_samplers: Option<&'a[D3D12StaticSamplerDescription]>, flags: D3D12RootSignatureFlags::Flags) -> Self {
        D3D12RootSignatureDescription {
            parameters: parameters,
            win32_parameters: None,
            static_samplers: static_samplers,
            win32_static_samplers: None,
            flags: flags,
        }
    }
}

impl<'a> From<&'a mut D3D12RootSignatureDescription<'a>> for winapi::D3D12_ROOT_SIGNATURE_DESC {
    fn from(source: &'a mut D3D12RootSignatureDescription<'a>) -> winapi::D3D12_ROOT_SIGNATURE_DESC {
        source.win32_parameters = Some(source.parameters.iter().map(|parameter| parameter.into()).collect());
        source.win32_static_samplers = match source.static_samplers {
            Some(static_samplers) => Some(static_samplers.iter().map(|sampler| sampler.into()).collect::<Vec<winapi::D3D12_STATIC_SAMPLER_DESC>>()),
            None => None,
        };
        winapi::D3D12_ROOT_SIGNATURE_DESC {
            NumParameters: match source.win32_parameters {
                Some(ref parameters) => parameters.len() as u32,
                None => unreachable!(),
            },
            pParameters: match source.win32_parameters {
                Some(ref parameters) => parameters.as_ptr(),
                None => unreachable!(),
            },
            NumStaticSamplers: match source.win32_static_samplers {
                Some(ref static_samplers) => static_samplers.len() as u32,
                None => 0,
            },
            pStaticSamplers: match source.win32_static_samplers {
                Some(ref static_samplers) => static_samplers.as_ptr(),
                None => std::ptr::null(),
            },
            Flags: winapi::D3D12_ROOT_SIGNATURE_FLAGS(source.flags.bits()),
        }
    }
}

pub struct D3D12StaticSamplerDescription {
    filter: D3D12Filter,
    address_u: D3D12TextureAddressMode,
    address_v: D3D12TextureAddressMode,
    address_w: D3D12TextureAddressMode,
    mip_lod_bias: f32,
    max_anisotropy: u32,
    comaprison_function: D3D12ComparisonFunction,
    border_color: D3D12StaticBorderColor,
    min_lod: f32,
    max_lod: f32,
    shader_register: u32,
    register_space: u32,
    shader_visibility: D3D12ShaderVisibility,
}

impl<'a> From<&'a D3D12StaticSamplerDescription> for winapi::D3D12_STATIC_SAMPLER_DESC {
    fn from(source: &'a D3D12StaticSamplerDescription) -> winapi::D3D12_STATIC_SAMPLER_DESC {
        winapi::D3D12_STATIC_SAMPLER_DESC {
            Filter: source.filter.into(),
            AddressU: source.address_u.into(),
            AddressV: source.address_v.into(),
            AddressW: source.address_w.into(),
            MipLODBias: source.mip_lod_bias,
            MaxAnisotropy: source.max_anisotropy,
            ComparisonFunc: source.comaprison_function.into(),
            BorderColor: source.border_color.into(),
            MinLOD: source.min_lod,
            MaxLOD: source.max_lod,
            ShaderRegister: source.shader_register,
            RegisterSpace: source.register_space,
            ShaderVisibility: source.shader_visibility.into(),
        }
    }
}

win32_enum! {
    enum D3D12Filter(winapi::D3D12_FILTER) {
        MinMagMipPoint = winapi::D3D12_FILTER_MIN_MAG_MIP_POINT,
        MinMagPointMipLinear = winapi::D3D12_FILTER_MIN_MAG_POINT_MIP_LINEAR,
        MinPointMagLinearMipPoint = winapi::D3D12_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT,
        MinPointMagMipLinear = winapi::D3D12_FILTER_MIN_POINT_MAG_MIP_LINEAR,
        MinLinearMagMipPoint = winapi::D3D12_FILTER_MIN_LINEAR_MAG_MIP_POINT,
        MinLinearMagPointMipLinear = winapi::D3D12_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
        MinMagLinearMipPoint = winapi::D3D12_FILTER_MIN_MAG_LINEAR_MIP_POINT,
        MinMagMipLinear = winapi::D3D12_FILTER_MIN_MAG_MIP_LINEAR,
        Anisotropic = winapi::D3D12_FILTER_ANISOTROPIC,
        ComparisonMinMagMipPoint = winapi::D3D12_FILTER_COMPARISON_MIN_MAG_MIP_POINT,
        ComparisonMinMagPointMipLinear = winapi::D3D12_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR,
        ComparisonMinPointMagLinearMipPoint = winapi::D3D12_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT,
        ComparisonMinPointMagMipLinear = winapi::D3D12_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR,
        ComparisonMinLinearMagMipPoint = winapi::D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT,
        ComparisonMinLinearMagPointMipLinear = winapi::D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
        ComparisonMinMagLinearMipPoint = winapi::D3D12_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT,
        ComparisonMinMagMipLinear = winapi::D3D12_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR,
        ComparisonAnisotropic = winapi::D3D12_FILTER_COMPARISON_ANISOTROPIC,
        MinimumMinMagMipPoint = winapi::D3D12_FILTER_MINIMUM_MIN_MAG_MIP_POINT,
        MinimumMinMagPointMipLinear = winapi::D3D12_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR,
        MinimumMinPointMagLinearMipPoint = winapi::D3D12_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT,
        MinimumMinPointMagMipLinear = winapi::D3D12_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR,
        MinimumMinLinearMagMipPoint = winapi::D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT,
        MinimumMinLinearMagPointMipLinear = winapi::D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
        MinimumMinMagLinearMipPoint = winapi::D3D12_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT,
        MinimumMinMagMipLinear = winapi::D3D12_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR,
        MinimumAnisotropic = winapi::D3D12_FILTER_MINIMUM_ANISOTROPIC,
        MaximumMinMagMipPoint = winapi::D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_POINT,
        MaximumMinMagPointMipLinear = winapi::D3D12_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR,
        MaximumMinPointMagLinearMipPoint = winapi::D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT,
        MaximumMinPointMagMipLinear = winapi::D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR,
        MaximumMinLinearMagMipPoint = winapi::D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT,
        MaximumMinLinearMagPointMipLinear = winapi::D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
        MaximumMinMagLinearMipPoint = winapi::D3D12_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT,
        MaximumMinMagMipLinear = winapi::D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR,
        MaximumAnisotropic = winapi::D3D12_FILTER_MAXIMUM_ANISOTROPIC,
    }
}

win32_enum! {
    enum D3D12TextureAddressMode(winapi::D3D12_TEXTURE_ADDRESS_MODE) {
        Wrap = winapi::D3D12_TEXTURE_ADDRESS_MODE_WRAP,
        Mirror = winapi::D3D12_TEXTURE_ADDRESS_MODE_MIRROR,
        Clamp = winapi::D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
        Border = winapi::D3D12_TEXTURE_ADDRESS_MODE_BORDER,
        MirrorOnce = winapi::D3D12_TEXTURE_ADDRESS_MODE_MIRROR_ONCE,
    }
}

win32_enum! {
    enum D3D12ComparisonFunction(winapi::D3D12_COMPARISON_FUNC) {
        Never = winapi::D3D12_COMPARISON_FUNC_NEVER,
        Less = winapi::D3D12_COMPARISON_FUNC_LESS,
        Equal = winapi::D3D12_COMPARISON_FUNC_EQUAL,
        LessEqual = winapi::D3D12_COMPARISON_FUNC_LESS_EQUAL,
        Greater = winapi::D3D12_COMPARISON_FUNC_GREATER,
        NotEqual = winapi::D3D12_COMPARISON_FUNC_NOT_EQUAL,
        GreaterEqual = winapi::D3D12_COMPARISON_FUNC_GREATER_EQUAL,
        Always = winapi::D3D12_COMPARISON_FUNC_ALWAYS,
    }
}

win32_enum! {
    enum D3D12StaticBorderColor(winapi::D3D12_STATIC_BORDER_COLOR) {
        TransparentBlack = winapi::D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK,
        OpaqueBlack = winapi::D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK,
        OpaqueWhite = winapi::D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE,
    }
}

// TODO(zeffron 2016-08-21): Figure out a better way to send back to error
// objects
pub fn d3d12_serialize_root_signature<'a>(
    root_signature: &'a mut D3D12RootSignatureDescription<'a>,
    version: D3DRootSignatureVersion,
) -> Result<ID3DBlob, (std::io::Error, Option<ID3DBlob>)>{
    let mut blob: *mut winapi::ID3DBlob = unsafe { std::mem::uninitialized() };
    let mut error: *mut winapi::ID3DBlob = unsafe { std::mem::uninitialized() };
    match unsafe { d3d12::D3D12SerializeRootSignature(
        &root_signature.into(),
        version.into(),
        &mut blob,
        &mut error,
    ) } {
        winapi::S_OK => Ok(blob.into()),
        result => panic!("{:x}", result),
    }
}

win32_enum! {
    enum D3DRootSignatureVersion(winapi::D3D_ROOT_SIGNATURE_VERSION) {
        Version1 = winapi::D3D_ROOT_SIGNATURE_VERSION_1,
        // TODO(zeffron 2016-08-19): Switch to winapi::D3D_ROOT_SIGNATURE_VERSION_1_0 once it exists
        Version10 = winapi::D3D_ROOT_SIGNATURE_VERSION(0x1),
        // TODO(zeffron 2016-08-19): Switch to winapi::D3D_ROOT_SIGNATURE_VERSION_1_1 once it exists
        Version11 = winapi::D3D_ROOT_SIGNATURE_VERSION(0x2),
    }
}

pub struct ID3D12RootSignature {
    ptr: *mut winapi::ID3D12RootSignature,
}

impl ID3D12RootSignature {
}

impl From<*mut std::os::raw::c_void> for ID3D12RootSignature {
    fn from(source: *mut std::os::raw::c_void) -> Self {
        ID3D12RootSignature {
            ptr: source as *mut winapi::ID3D12RootSignature,
        }
    }
}

impl Deref for ID3D12RootSignature {
    type Target=ID3D12DeviceChild;

    fn deref(&self) -> &ID3D12DeviceChild {
        unsafe { &*(self as *const ID3D12RootSignature as *const ID3D12DeviceChild) }
    }
}

impl DeclspecUUID for ID3D12RootSignature {
    fn uuid() -> winapi::GUID {
        dxguid::IID_ID3D12RootSignature
    }
}