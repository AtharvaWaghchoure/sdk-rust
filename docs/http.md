# HTTP Extension

The HTTP extension in the Blockless Protocol Rust SDK provides a robust interface for making HTTP requests and handling responses directly from Rust applications. This feature allows you to open HTTP connections, configure request options, and efficiently manage request and response data.

### Key Components

- **HttpOptions**: Configures HTTP request options such as method, timeouts, and body content.
- **BlocklessHttp**: Manages HTTP connections, handles request execution, and retrieves response data.

### 1. HttpOptions

`HttpOptions` provides a way to specify settings for HTTP requests.

- **Fields**:
  - `method`: HTTP method (e.g., GET, POST).
  - `connect_timeout`: Connection timeout in milliseconds.
  - `read_timeout`: Read timeout in milliseconds.
  - `body`: Optional request body.

- **Methods**:
  - `new(method: &str, connect_timeout: u32, read_timeout: u32)`: Constructs a new `HttpOptions` instance.
  - `dump(&self)`: Serializes the HTTP options to a JSON string for use in requests.

### 2. BlocklessHttp

`BlocklessHttp` is the primary struct for handling HTTP operations.

- **Fields**:
  - `inner`: Internal handle for managing the HTTP connection.
  - `code`: Status code from the HTTP response.

- **Methods**:
  - `open(url: &str, opts: &HttpOptions) -> Result<Self, HttpErrorKind>`: Opens an HTTP connection to the specified URL with the given options. Returns a `BlocklessHttp` instance on success.
  - `get_code(&self) -> CodeStatus`: Retrieves the status code of the HTTP response.
  - `get_all_body(&self) -> Result<Vec<u8>, HttpErrorKind>`: Reads the entire body of the HTTP response.
  - `get_header(&self, header: &str) -> Result<String, HttpErrorKind>`: Retrieves the value of a specified HTTP header.
  - `close(self)`: Closes the HTTP connection.
  - `read_body(&self, buf: &mut [u8]) -> Result<u32, HttpErrorKind>`: Reads a portion of the HTTP response body into the provided buffer.

### Error Handling

The HTTP operations return `Result` types, with possible errors encapsulated in `HttpErrorKind`.

