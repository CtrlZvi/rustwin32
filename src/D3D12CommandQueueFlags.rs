// TOOD(zeffron 2016-08-18): We want to use Pascal-case names, but bitflags
// prevents us from cleaning up the warnings.
bitflags! {
    pub flags Flags: u32 {
        const NONE = 0,
        const DISABLE_GPU_TIMEOUT = 1,
    }
}