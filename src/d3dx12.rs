// Wrappers around items provided by D3dx12.h
use d3d12::*;

use D3D12RootSignatureFlags;

pub struct CD3DX12DescriptorRange;

impl CD3DX12DescriptorRange {
     pub fn init(
         range_type: D3D12DescriptorRangeType,
         num_descriptors: u32,
         base_shader_register: u32,
         register_space: u32,
         offset_in_descriptors_from_table_start: u32,
    ) -> D3D12DescriptorRange {
        D3D12DescriptorRange {
            range_type: range_type,
            num_descriptors: num_descriptors,
            base_shader_register: base_shader_register,
            register_space: register_space,
            offset_in_descriptors_from_table_start: offset_in_descriptors_from_table_start,
        }
    }
}

pub struct CD3DX12RootParameter;

impl CD3DX12RootParameter {
    pub fn init_as_descriptor_table(
        descriptor_ranges: &[D3D12DescriptorRange],
        visibility: D3D12ShaderVisibility,
    ) -> D3D12RootParameter {
        D3D12RootParameter {
            parameter_type: D3D12RootParameterType::DescriptorTable,
            data: D3D12RootParameterData::DescriptorTable(D3D12RootDescriptorTable {
                ranges: descriptor_ranges,
            }),
            shader_visibility: visibility,
        }
    }

    pub fn init_as_constants<'a>(
        num_32bit_values: u32,
        shader_register: u32,
        register_space: u32,
        visibility: D3D12ShaderVisibility,
    ) -> D3D12RootParameter<'a> {
        D3D12RootParameter {
            parameter_type: D3D12RootParameterType::Constants32Bit,
            data: D3D12RootParameterData::Constants(D3D12RootConstants {
                shader_register: shader_register,
                register_space: register_space,
                num_32bit_values: num_32bit_values,
            }),
            shader_visibility: visibility,
        }
    }

    pub fn init_as_constant_buffer_view<'a>(
        shader_register: u32,
        register_space: u32,
        visibility: D3D12ShaderVisibility,
    ) -> D3D12RootParameter<'a> {
        D3D12RootParameter {
            parameter_type: D3D12RootParameterType::CBV,
            data: D3D12RootParameterData::Descriptor(D3D12RootDescriptor {
                shader_register: shader_register,
                register_space: register_space,
            }),
            shader_visibility: visibility,
        }
    }

    pub fn init_as_shader_resource_view<'a>(
        shader_register: u32,
        register_space: u32,
        visibility: D3D12ShaderVisibility,
    ) -> D3D12RootParameter<'a> {
        D3D12RootParameter {
            parameter_type: D3D12RootParameterType::SRV,
            data: D3D12RootParameterData::Descriptor(D3D12RootDescriptor {
                shader_register: shader_register,
                register_space: register_space,
            }),
            shader_visibility: visibility,
        }
    }

    pub fn init_as_unordered_access_view<'a>(
        shader_register: u32,
        register_space: u32,
        visibility: D3D12ShaderVisibility,
    ) -> D3D12RootParameter<'a> {
        D3D12RootParameter {
            parameter_type: D3D12RootParameterType::UAV,
            data: D3D12RootParameterData::Descriptor(D3D12RootDescriptor {
                shader_register: shader_register,
                register_space: register_space,
            }),
            shader_visibility: visibility,
        }
    }
}

pub struct CD3DX12RootSignatureDescription;

impl CD3DX12RootSignatureDescription {
    pub fn init<'a>(
        parameters: &'a[D3D12RootParameter],
        static_samplers: Option<&'a[D3D12StaticSamplerDescription]>,
        flags: D3D12RootSignatureFlags::Flags,
    ) -> D3D12RootSignatureDescription<'a> {
        D3D12RootSignatureDescription {
            parameters: parameters,
            static_samplers: static_samplers,
            flags: flags,
        }
    }
}