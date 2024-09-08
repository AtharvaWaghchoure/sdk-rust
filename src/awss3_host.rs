#[link(wasm_import_module = "blockless_s3")]
extern "C" {
    #[link_name = "bucket_command"]
    pub(crate) fn bucket_command(cmd: u16, opts: *const u8, opts_len: u32, fd: *mut u32) -> u32;

    #[link_name = "s3_read"]
    pub(crate) fn s3_read(h: u32, buf: *mut u32, len: u32, ptr: *mut u32) -> u32;

    #[link_name = "bucket_put_object"]
    pub(crate) fn bucket_put_object(opts: *const u8, opts_len: u32, buf: *mut u32, len: u32)
        -> u32;

    #[link_name = "s3_close"]
    pub(crate) fn s3_close(h: u32) -> u32;
}
