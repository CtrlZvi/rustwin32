// Wrappers around items provided by D3dx12.h
use d3d12::*;

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