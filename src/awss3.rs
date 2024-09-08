use json::{object, JsonValue};

use crate::{
    awss3_host::{bucket_command, bucket_put_object, s3_close, s3_read},
    AWSS3ErrorKind,
};

fn read_body(h: u32, buf: &mut [u8]) -> Result<usize, AWSS3ErrorKind> {
    let mut num: u32 = 0;
    let rs = unsafe { s3_read(h, buf.as_mut_ptr() as *mut u32, buf.len() as u32, &mut num) };
    if rs != 0 {
        return Err(AWSS3ErrorKind::from(rs));
    }
    Ok(num as usize)
}

fn get_body(h: u32) -> Result<Vec<u8>, AWSS3ErrorKind> {
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

#[derive(Debug)]
pub struct BucketListContent {
    pub last_modified: String,
    pub e_tag: Option<String>,
    pub storage_class: Option<String>,
    pub key: String,
    pub size: u64,
}

impl std::fmt::Display for BucketListContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bl_content = format!(
            "last_modified:{}, key:{}, size:{}",
            self.last_modified, self.key, self.size
        );
        if self.e_tag.is_some() {
            let e_tag = format!(", e_tag:{}", self.e_tag.clone().unwrap());
            bl_content.push_str(e_tag.as_str());
        }
        if self.storage_class.is_some() {
            let s_class = format!(", storage_class:{}", self.storage_class.clone().unwrap());
            bl_content.push_str(s_class.as_str());
        }
        write!(f, "{}", bl_content)
    }
}

#[derive(Debug)]
pub struct BucketListResponse {
    pub name: String,
    pub prefix: Option<String>,
    pub is_truncated: bool,
    pub contents: Vec<BucketListContent>,
}

impl BucketListResponse {
    pub fn new(name: String, prefix: Option<String>) -> Self {
        BucketListResponse {
            name,
            prefix,
            is_truncated: false,
            contents: Vec::new(),
        }
    }

    pub fn set_truncated(&mut self, is_truncated: bool) {
        self.is_truncated = is_truncated;
    }
}

impl std::fmt::Display for BucketListResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bl_response = format!("name:{}, is_truncated:{}", self.name, self.is_truncated);
        if self.prefix.is_some() {
            let prefix = format!(", prefix:{}", self.prefix.clone().unwrap());
            bl_response.push_str(prefix.as_str());
        }
        let contents = format!(", contents: {:?}", self.contents);
        bl_response.push_str(contents.as_str());
        write!(f, "{}", bl_response)
    }
}

#[derive(Clone)]
pub struct AWSS3Configure {
    pub access_key: String,
    pub secret_key: String,
    pub endpoint: String,
    pub region: Option<String>,
}

impl AWSS3Configure {
    pub fn new(access_key: String, secret_key: String, endpoint: String) -> Self {
        AWSS3Configure {
            access_key,
            secret_key,
            endpoint,
            region: None,
        }
    }

    pub fn set_region(&mut self, region: String) {
        self.region = Some(region);
    }

    pub fn encode_json(&self) -> String {
        let mut obj = object! {
            "access_key" : self.access_key.clone(),
            "secret_key": self.secret_key.clone(),
            "endpoint": self.endpoint.clone(),
        };

        if let Some(region) = &self.region {
            obj["region"] = json::JsonValue::String(region.clone());
        }

        obj.dump()
    }
}

pub struct Args {
    name: String,
    value: String,
}

impl Args {
    fn new(name: String, value: String) -> Self {
        Args { name, value }
    }
}

pub struct BucketCommand {
    pub config: AWSS3Configure,
    pub args: Vec<Args>,
}

impl BucketCommand {
    pub fn new(config: AWSS3Configure) -> Self {
        BucketCommand {
            config,
            args: Vec::new(),
        }
    }

    pub fn add_arg(&mut self, name: String, value: String) {
        self.args.push(Args::new(name, value))
    }

    pub fn to_json(&self) -> String {
        let mut json_value =
            json::parse(&self.config.encode_json()).unwrap_or(json::JsonValue::Null);

        if let JsonValue::Object(obj) = &mut json_value {
            for arg in &self.args {
                obj[&arg.name] = JsonValue::String(arg.value.clone());
            }
        }
        json_value.dump()
    }
}

#[derive(Clone)]
pub struct Bucket {
    pub bucket_name: String,
    pub s3config: AWSS3Configure,
}

impl Bucket {
    pub fn new(bucket_name: String, s3config: AWSS3Configure) -> Self {
        Bucket {
            bucket_name,
            s3config,
        }
    }

    pub fn list(self, prefix: &str) -> Result<Vec<BucketListResponse>, AWSS3ErrorKind> {
        let mut cmd = self.get_bucket_command();
        cmd.add_arg("prefix".to_string(), prefix.to_string());
        let command = cmd.to_json();
        let mut handle: u32 = 0;
        let rs = unsafe { bucket_command(2, command.as_ptr(), command.len() as u32, &mut handle) };
        if rs != 0 {
            return Err(AWSS3ErrorKind::from(rs));
        }
        let body = get_body(handle)?;
        let body_str = String::from_utf8(body).map_err(|_| AWSS3ErrorKind::Utf8Error)?;

        let json_value = json::parse(&body_str).map_err(|_| AWSS3ErrorKind::InvalidEncoding)?;
        let mut result = Vec::new();

        if let JsonValue::Array(array) = json_value {
            for item in array {
                if let JsonValue::Object(obj) = item {
                    let mut response = BucketListResponse::new(
                        obj["name"].as_str().unwrap_or("").to_string(),
                        obj["prefix"].as_str().map(|s| s.to_string()),
                    );

                    response.set_truncated(obj["is_truncated"].as_bool().unwrap_or(false));

                    if let JsonValue::Array(contents) = &obj["contents"] {
                        for content in contents {
                            if let JsonValue::Object(content_obj) = content {
                                let content_item = BucketListContent {
                                    last_modified: content_obj["last_modified"]
                                        .as_str()
                                        .unwrap_or("")
                                        .to_string(),
                                    e_tag: content_obj["e_tag"].as_str().map(|s| s.to_string()),
                                    storage_class: content_obj["storage_class"]
                                        .as_str()
                                        .map(|s| s.to_string()),
                                    key: content_obj["key"].as_str().unwrap_or("").to_string(),
                                    size: content_obj["size"].as_u64().unwrap_or(0),
                                };
                                response.contents.push(content_item);
                            }
                        }
                    }
                    result.push(response);
                }
            }
        }
        let rs = unsafe { s3_close(handle) };
        if rs != 0 {
            return Err(AWSS3ErrorKind::from(rs));
        }

        Ok(result)
    }

    pub fn get_bucket_command(self) -> BucketCommand {
        let mut cmd = BucketCommand::new(self.s3config);
        cmd.add_arg("bucket_name".to_string(), self.bucket_name);
        cmd
    }

    pub fn get_object(self, path: &str) -> Result<Vec<u8>, AWSS3ErrorKind> {
        let mut cmd = self.get_bucket_command();
        cmd.add_arg("path".to_string(), path.to_string());
        let command = cmd.to_json();
        let mut handle: u32 = 0;

        let rs = unsafe { bucket_command(3, command.as_ptr(), command.len() as u32, &mut handle) };

        if rs != 0 {
            return Err(AWSS3ErrorKind::from(rs));
        }
        let bs = get_body(handle)?;
        let rs = unsafe { s3_close(handle) };
        if rs != 0 {
            return Err(AWSS3ErrorKind::from(rs));
        }
        Ok(bs)
    }

    pub fn put_object(self, path: &str, content: &mut [u8]) -> Result<bool, AWSS3ErrorKind> {
        let mut cmd = self.get_bucket_command();
        cmd.add_arg("path".to_string(), path.to_string());
        let command = cmd.to_json();

        let rs = unsafe {
            bucket_put_object(
                command.as_ptr(),
                command.len() as u32,
                content.as_mut_ptr() as *mut u32,
                content.len() as u32,
            )
        };

        if rs != 0 {
            return Err(AWSS3ErrorKind::from(rs));
        }
        Ok(true)
    }

    pub fn delete_object(self, path: &str) -> Result<bool, AWSS3ErrorKind> {
        let mut cmd = self.get_bucket_command();
        cmd.add_arg("path".to_string(), path.to_string());
        let command = cmd.to_json();
        let mut handle = 0u32;

        let rs = unsafe { bucket_command(4, command.as_ptr(), command.len() as u32, &mut handle) };

        if rs != 0 {
            println!("Error in bucket_command: {}", rs);
            return Err(AWSS3ErrorKind::from(rs));
        }

        let rs = unsafe { s3_close(handle) };
        if rs != 0 {
            println!("Error in s3_close: {}", rs);
            return Err(AWSS3ErrorKind::from(rs));
        }
        Ok(true)
    }
}
