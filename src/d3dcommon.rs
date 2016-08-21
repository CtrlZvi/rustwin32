// Wrappers around items provided by D3DCommon.h
use winapi;

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