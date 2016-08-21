// TOOD(zeffron 2016-08-18): We want to use Pascal-case names, but bitflags
// prevents us from cleaning up the warnings.
bitflags! {
    pub flags Flags: u32 {
        const NONE = 0,
        const ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT = 0x1,
        const DENY_VERTEX_SHADER_ROOT_ACCESS = 0x2,
        const DENY_HULL_SHADER_ROOT_ACCESS = 0x4,
        const DENY_DOMAIN_SHADER_ROOT_ACCESS = 0x8,
        const DENY_GEOMETRY_SHADER_ROOT_ACCESS = 0x10,
        const DENY_PIXEL_SHADER_ROOT_ACCESS = 0x20,
        const ALLOW_STREAM_OUTPUT = 0x40,
    }
}