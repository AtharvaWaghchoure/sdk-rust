# IPFS Extension

The Blockless Rust SDK's IPFS extension allows you to interact with the IPFS network from your rust code. 
This feature provides comprehensive capabilities for managing IPFS files and directories, including creating, reading, writing, listing, and deleting files. 


### `File`
A struct representing an IPFS file.
- **Fields**: `name`, `file_type`, `size`, `hash`.

### `FileStat`
A struct for fill up your file information
- **Fields**: `hash`, `size`, `blocks`, `file_type`, `cumulative_size`.

### `FileWriteOptions`
A struct for configuring file write operations.
- **Fields**: `file`, `offset`, `create`, `parents`, `truncate`.



### File Operations
- **`ipfs_create_dir(path: &str, parents: bool)`**: Creates a directory at `path`.
- **`ipfs_file_remove(path: &str, recursive: bool, force: bool)`**: Removes a file or directory.
- **`ipfs_file_copy(src: &str, dst: &str, parents: bool)`**: Copies a file from `src` to `dst`.
- **`ipfs_file_read(path: &str, offset: u64, buf: &mut [u8])`**: Reads a file at `path` starting from `offset`.
- **`ipfs_file_write(wopts: &FileWriteOptions, buf: &mut [u8])`**: Writes data to a file.
- **`ipfs_file_list(path: Option<&str>)`**: Lists files in a directory.
- **`ipfs_file_stat(path: &str)`**: Gets statistics of a file.
- **`read_entire_file(path: &str)`**: Reads the entire file into a buffer.


### Error Handling

The operations return `Result` types, with possible errors wrapped in `IPFSErrorKind`.

### Examples
- [ipfs_example](../examples/ipfs_example.rs)
