// Wrappers around items provided by dxgitype.h
use winapi;

pub struct DXGISampleDescription {
    pub count: u32,
    pub quality: u32,
}

impl From<DXGISampleDescription> for winapi::DXGI_SAMPLE_DESC {
    fn from(source: DXGISampleDescription) -> winapi::DXGI_SAMPLE_DESC {
        winapi::DXGI_SAMPLE_DESC {
            Count: source.count,
            Quality: source.quality,
        }
    }
}