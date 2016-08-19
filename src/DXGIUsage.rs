use winapi;

// TOOD(zeffron 2016-08-18): We want to use Pascal-case names, but bitflags
// prevents us from cleaning up the warnings.
bitflags! {
    pub flags Flags: u32 {
        const BACK_BUFFER = 1 << 6,
        const DISCARD_ON_PRESENT = 1 << 9,
        const READ_ONLY = 1 << 8,
        const RENDER_TARGET_OUTPUT = 1 << 5,
        const SHADER_INPUT = 1 << 4,
        const SHARED = 1 << 7,
        const UNORDERED_ACCESS = 1 << 10,
    }
}