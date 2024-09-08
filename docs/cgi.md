# CGI Extension

The Blockless Rust SDK's CGI extension allows you to interact with programs or library written in other programming languages.

### Key Components

- **CGIExtensions**: Details about a CGI extension (file name, alias, MD5, description).
- **CGIEnv**: Represents an environment variable for CGI commands.
- **CGICommand**: Manages and executes CGI commands.
- **CGIListExtensions**: Lists available CGI extensions and creates `CGICommand` instances.

### 1. CGIExtensions

Represents a CGI extension with:
- `file_name`: File name of the CGI extension.
- `alias`: Alias for the extension.
- `md5`: MD5 checksum.
- `description`: Description of the extension.

### 2. CGIEnv

Represents environment variables:
- `name`: Variable name.
- `value`: Variable value.

### 3. CGICommand

Handles CGI command execution:
- `new(command: String, args: Vec<String>, envs: Vec<CGIEnv>) -> Self`: Create a new command instance.
- `exec(&mut self) -> Result<(), CGIErrorKind>`: Execute the command.
- `read_all_stdin(&mut self) -> Result<Vec<u8>, CGIErrorKind>`: Read standard output.
- `read_all_stderr(&mut self) -> Result<Vec<u8>, CGIErrorKind>`: Read standard error.
- `exec_command(&mut self) -> Result<String, CGIErrorKind>`: Execute command and get output as a string.

### 4. CGIListExtensions

Lists and manages CGI extensions:
- `new() -> Result<Self, CGIErrorKind>`: Create a new list instance.
- `list(&self) -> Result<Vec<CGIExtensions>, CGIErrorKind>`: List all extensions.
- `command(&self, command: &str, args: Vec<String>, envs: Vec<CGIEnv>) -> Result<CGICommand, CGIErrorKind>`: Get a `CGICommand` by alias.

### Error Handling

Operations return `Result` types with errors represented by `CGIErrorKind`.

