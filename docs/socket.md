# Socket Host Extension

The Socket Host extension in the Blockless Protocol Rust SDK provides functionality for creating and managing TCP sockets. 

### Key Function

#### `create_tcp_bind_socket(addr: &str) -> Result<u32, SocketErrorKind>`

Creates a TCP socket bound to the specified address.


### Error Handling
The operations return `Result` types, with possible errors wrapped in `SocketErrorKind`.

