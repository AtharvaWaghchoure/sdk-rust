use json::{self, JsonValue};

use crate::{
    ipfs_host::{ipfs_close, ipfs_command, ipfs_read, ipfs_write},
    IPFSErrorKind,
};

struct IPFSOptions {
    api: String,
    args: Vec<Arg>,
}

// #[derive(Debug, Serialize, Deserialize)]
struct Arg {
    name: String,
    value: String,
}

struct CommandResult {
    status_code: u32,
    resp_body: Option<Vec<u8>>,
}

impl IPFSOptions {
    fn new(api: &str) -> Self {
        IPFSOptions {
            api: api.to_string(),
            args: Vec::new(),
        }
    }

    fn add_arg(&mut self, name: &str, value: &str) {
        self.args.push(Arg {
            name: name.to_string(),
            value: value.to_string(),
        });
    }

    fn to_json(&self) -> String {
        json::object! {
            api: self.api.clone(),
            args: self.args.iter().map(|arg| json::object! {
                name: arg.name.clone(),
                value: arg.value.clone()
            }).collect::<Vec<JsonValue>>()
        }
        .dump()
    }
}

fn ipfs_command_result(opts: &IPFSOptions) -> Result<CommandResult, IPFSErrorKind> {
    let result = ipfs_command_raw(opts)?;
    let body = get_body(result.handle)?;
    unsafe {
        if ipfs_close(result.handle) != 0 {
            return Err(IPFSErrorKind::InvalidHandle);
        }
    };
    Ok(CommandResult {
        status_code: result.code,
        resp_body: Some(body),
    })
}

struct CommandRs {
    code: u32,
    handle: u32,
}

fn ipfs_command_raw(opts: &IPFSOptions) -> Result<CommandRs, IPFSErrorKind> {
    let opts_json = opts.to_json();
    let mut handle: u32 = 0;
    let mut code: u32 = 0;

    let rs = unsafe {
        ipfs_command(
            opts_json.as_ptr(),
            opts_json.len() as u32,
            &mut handle,
            &mut code,
        )
    };

    if rs != 0 {
        return Err(IPFSErrorKind::from(rs));
    }

    Ok(CommandRs { code, handle })
}

fn write_body(h: u32, buf: &mut [u8]) -> Result<usize, IPFSErrorKind> {
    let mut num: u32 = 0;
    let rs = unsafe { ipfs_write(h, buf.as_mut_ptr() as *mut u32, buf.len() as u32, &mut num) };

    if rs != 0 {
        return Err(IPFSErrorKind::from(rs));
    }

    Ok(num as usize)
}

fn read_body(h: u32, buf: &mut [u8]) -> Result<usize, IPFSErrorKind> {
    let mut num: u32 = 0;
    let rs = unsafe { ipfs_read(h, buf.as_mut_ptr() as *mut u32, buf.len() as u32, &mut num) };
    if rs != 0 {
        return Err(IPFSErrorKind::from(rs));
    }

    Ok(num as usize)
}

fn get_body(h: u32) -> Result<Vec<u8>, IPFSErrorKind> {
    let mut result = Vec::new();
    let mut buf = vec![0; 1024];
    loop {
        let num = read_body(h, &mut buf)?;
        if num == 0 {
            break;
        }
        result.extend_from_slice(&buf[..num]);
    }
    Ok(result)
}

pub struct File {
    name: String,
    file_type: i64,
    size: i64,
    hash: String,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name:{}, size:{}, type:{}, hash:{}",
            self.name, self.size, self.file_type, self.hash
        )
    }
}

pub fn ipfs_create_dir(path: &str, parents: bool) -> Result<bool, IPFSErrorKind> {
    let mut opts = IPFSOptions::new("files/mkdir");
    opts.add_arg("arg", path);
    opts.add_arg("parents", &parents.to_string());
    let result = ipfs_command_result(&opts)?;
    Ok(result.status_code == 200)
}

pub fn ipfs_file_remove(path: &str, recursive: bool, force: bool) -> Result<bool, IPFSErrorKind> {
    let mut opts = IPFSOptions::new("files/rm");
    opts.add_arg("arg", path);
    opts.add_arg("recursive", &recursive.to_string());
    opts.add_arg("force", &force.to_string());
    let result = ipfs_command_result(&opts)?;
    Ok(result.status_code == 200)
}

pub fn ipfs_file_copy(src: &str, dst: &str, parents: bool) -> Result<bool, IPFSErrorKind> {
    let mut opts = IPFSOptions::new("files/cp");
    opts.add_arg("arg", src);
    opts.add_arg("arg", dst);
    opts.add_arg("parents", &parents.to_string());
    let result = ipfs_command_result(&opts)?;
    Ok(result.status_code == 200)
}

pub struct FileWriteOptions {
    pub file: String,
    pub offset: i64,
    pub create: bool,
    pub parents: bool,
    pub truncate: bool,
}

impl FileWriteOptions {
    pub fn new(file: &str) -> Self {
        FileWriteOptions {
            file: file.to_string(),
            offset: 0,
            create: true,
            parents: false,
            truncate: false,
        }
    }
}

pub fn ipfs_file_read(path: &str, offset: u64, buf: &mut [u8]) -> Result<usize, IPFSErrorKind> {
    let mut opts = IPFSOptions::new("files/read");
    opts.add_arg("arg", path);
    opts.add_arg("offset", &offset.to_string());
    opts.add_arg("count", &buf.len().to_string());
    let rs = ipfs_command_raw(&opts)?;
    let num = read_body(rs.handle, buf)?;
    unsafe { ipfs_close(rs.handle) };
    Ok(num)
}

pub fn ipfs_file_write(wopts: &FileWriteOptions, buf: &mut [u8]) -> Result<bool, IPFSErrorKind> {
    let mut opts = IPFSOptions::new("files/write");
    opts.add_arg("arg", &wopts.file);
    opts.add_arg("offset", &wopts.offset.to_string());
    opts.add_arg("create", &wopts.create.to_string());
    opts.add_arg("parents", &wopts.parents.to_string());
    opts.add_arg("truncate", &wopts.truncate.to_string());
    let rs = ipfs_command_raw(&opts)?;
    write_body(rs.handle, buf)?;
    let _rbuf = get_body(rs.handle)?;
    unsafe { ipfs_close(rs.handle) };
    Ok(true)
}

pub fn ipfs_file_list(path: Option<&str>) -> Result<Vec<File>, IPFSErrorKind> {
    let mut opts = IPFSOptions::new("files/ls");
    if let Some(p) = path {
        opts.add_arg("args", p);
    }
    let result = ipfs_command_result(&opts)?;
    if result.status_code != 200 {
        return Err(IPFSErrorKind::RequestError);
    }
    let json_str =
        String::from_utf8(result.resp_body.unwrap()).map_err(|_| IPFSErrorKind::Utf8Error)?;
    let parsed = json::parse(&json_str).map_err(|_| IPFSErrorKind::InvalidParameter)?;
    if !parsed["Entries"].is_array() {
        return Err(IPFSErrorKind::InvalidParameter);
    }
    let files: Vec<File> = parsed["Entries"]
        .members()
        .filter_map(|v| {
            Some(File {
                name: v["Name"].as_str()?.to_string(),
                file_type: v["Type"].as_i64()?,
                size: v["Size"].as_i64()?,
                hash: v["Hash"].as_str()?.to_string(),
            })
        })
        .collect();
    Ok(files)
}

pub struct FileStat {
    hash: String,
    size: u64,
    blocks: u64,
    file_type: String,
    cumulative_size: u64,
}

impl std::fmt::Display for FileStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "hash:{}, size:{}, blocks:{}, type:{}, cumulative size:{}",
            self.hash, self.size, self.blocks, self.file_type, self.cumulative_size
        )
    }
}

pub fn ipfs_file_stat(path: &str) -> Result<FileStat, IPFSErrorKind> {
    let mut opts = IPFSOptions::new("files/stat");
    opts.add_arg("arg", path);
    let result = ipfs_command_result(&opts)?;
    if result.status_code != 200 {
        return Err(IPFSErrorKind::RequestError);
    }
    let json_str =
        String::from_utf8(result.resp_body.unwrap()).map_err(|_| IPFSErrorKind::Utf8Error)?;
    let parsed = json::parse(&json_str).map_err(|_| IPFSErrorKind::InvalidParameter)?;
    Ok(FileStat {
        hash: parsed["Hash"]
            .as_str()
            .ok_or(IPFSErrorKind::InvalidParameter)?
            .to_string(),
        size: parsed["Size"]
            .as_u64()
            .ok_or(IPFSErrorKind::InvalidParameter)?,
        blocks: parsed["Blocks"]
            .as_u64()
            .ok_or(IPFSErrorKind::InvalidParameter)?,
        file_type: parsed["Type"]
            .as_str()
            .ok_or(IPFSErrorKind::InvalidParameter)?
            .to_string(),
        cumulative_size: parsed["CumulativeSize"]
            .as_u64()
            .ok_or(IPFSErrorKind::InvalidParameter)?,
    })
}

pub fn read_entire_file(path: &str) -> Result<Vec<u8>, IPFSErrorKind> {
    let mut content = Vec::new();
    let mut offset = 0;
    loop {
        let mut buffer = vec![0u8; 1024]; // Read in 1KB chunks
        match ipfs_file_read(path, offset, &mut buffer) {
            Ok(0) => break, // End of file
            Ok(bytes_read) => {
                content.extend_from_slice(&buffer[..bytes_read]);
                offset += bytes_read as u64;
            }
            Err(e) => return Err(e),
        }
    }
    Ok(content)
}
