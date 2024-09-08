#[link(wasm_import_module = "blockless_ipfs")]
extern "C" {
    #[link_name = "ipfs_command"]
    pub(crate) fn ipfs_command(
        opts: *const u8,
        opts_len: u32,
        fd: *mut u32,
        status: *mut u32,
    ) -> u32;

    #[link_name = "ipfs_read"]
    pub(crate) fn ipfs_read(h: u32, buf: *mut u32, len: u32, ptr: *mut u32) -> u32;

    #[link_name = "ipfs_write"]
    pub(crate) fn ipfs_write(h: u32, buf: *mut u32, len: u32, ptr: *mut u32) -> u32;

    #[link_name = "ipfs_close"]
    pub(crate) fn ipfs_close(h: u32) -> u32;
}
