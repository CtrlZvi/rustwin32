// TOOD(zeffron 2016-08-18): We want to use Pascal-case names, but bitflags
// prevents us from cleaning up the warnings.
bitflags! {
    pub flags Flags: u32 {
        const NONPREROTATED = 1,
        const ALLOW_MODE_SWITCH = 2,
        const GDI_COMPATIBLE = 4,
        const RESTRICTED_CONTENT = 8,
        const RESTRICT_SHARED_RESOURCE_DRIVER = 16,
        const DISPLAY_ONLY = 32,
        const FRAME_LATENCY_WAITABLE_OBJECT = 64,
        const FOREGROUND_LAYER = 128,
        const FULLSCREEN_VIDEO = 256,
        const YUV_VIDEO = 512,
        const HW_PROTECTED = 1024,
        const ALLOW_TEARING = 2048,
    }
}