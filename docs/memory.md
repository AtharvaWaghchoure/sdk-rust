# Memory Host Extension

The Memory Host extension in the Blockless Protocol Rust SDK enables reading from standard input and environment variables directly from Rust applications.

### Functions

#### 1. `read_stdin(buf: &mut [u8]) -> std::io::Result<u32>`

Reads data from standard input.

#### 2. `read_env_vars(buf: &mut [u8]) -> std::io::Result<u32>`

Reads data from environment variables.

- **Description**:
  Uses `env_var_read` to fill the buffer with environment variable data.

### Error Handling

Errors are wrapped with standard io error crate `std::io::Error`.

