#[derive(Debug)]
pub enum HttpErrorKind {
    InvalidDriver,
    InvalidHandle,
    MemoryAccessError,
    BufferTooSmall,
    HeaderNotFound,
    Utf8Error,
    DestinationNotAllowed,
    InvalidMethod,
    InvalidEncoding,
    InvalidUrl,
    RequestError,
    RuntimeError,
    TooManySessions,
    PermissionDeny,
}

impl std::error::Error for HttpErrorKind {}

impl std::fmt::Display for HttpErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidDriver => write!(f, "Invalid Driver"),
            Self::InvalidHandle => write!(f, "Invalid Error"),
            Self::MemoryAccessError => write!(f, "Memoery Access Error"),
            Self::BufferTooSmall => write!(f, "Buffer too small"),
            Self::HeaderNotFound => write!(f, "Header not found"),
            Self::Utf8Error => write!(f, "Utf8 error"),
            Self::DestinationNotAllowed => write!(f, "Destination not allowed"),
            Self::InvalidMethod => write!(f, "Invalid method"),
            Self::InvalidEncoding => write!(f, "Invalid encoding"),
            Self::InvalidUrl => write!(f, "Invalid url"),
            Self::RequestError => write!(f, "Request url"),
            Self::RuntimeError => write!(f, "Runtime error"),
            Self::TooManySessions => write!(f, "Too many sessions"),
            Self::PermissionDeny => write!(f, "Permision deny."),
        }
    }
}

impl From<u32> for HttpErrorKind {
    fn from(i: u32) -> HttpErrorKind {
        match i {
            1 => HttpErrorKind::InvalidHandle,
            2 => HttpErrorKind::MemoryAccessError,
            3 => HttpErrorKind::BufferTooSmall,
            4 => HttpErrorKind::HeaderNotFound,
            5 => HttpErrorKind::Utf8Error,
            6 => HttpErrorKind::DestinationNotAllowed,
            7 => HttpErrorKind::InvalidMethod,
            8 => HttpErrorKind::InvalidEncoding,
            9 => HttpErrorKind::InvalidUrl,
            10 => HttpErrorKind::RequestError,
            11 => HttpErrorKind::RuntimeError,
            12 => HttpErrorKind::TooManySessions,
            13 => HttpErrorKind::PermissionDeny,
            _ => HttpErrorKind::RuntimeError,
        }
    }
}

#[derive(Debug)]
pub enum SocketErrorKind {
    ConnectRefused,
    ParameterError,
    ConnectionReset,
    AddressInUse,
}

impl std::fmt::Display for SocketErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SocketErrorKind::ConnectRefused => write!(f, "Connect Refused."),
            SocketErrorKind::ParameterError => write!(f, "Parameter Error."),
            SocketErrorKind::ConnectionReset => write!(f, "Connection  Reset."),
            SocketErrorKind::AddressInUse => write!(f, "Address In Use."),
        }
    }
}

impl std::error::Error for SocketErrorKind {}

#[derive(Debug)]
pub enum CGIErrorKind {
    ListError,
    EncodingError,
    JsonDecodingError,
    ExecError,
    ReadError,
    NoCommandError,
}

impl std::fmt::Display for CGIErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            CGIErrorKind::ListError => write!(f, "CGI List Error."),
            CGIErrorKind::EncodingError => write!(f, "CGI Encoding Error."),
            CGIErrorKind::JsonDecodingError => write!(f, "Json decoding Error."),
            CGIErrorKind::ExecError => write!(f, "CGI Exec Error."),
            CGIErrorKind::ReadError => write!(f, "Read Error."),
            CGIErrorKind::NoCommandError => write!(f, "No CGI Command Error."),
        }
    }
}

impl std::error::Error for CGIErrorKind {}

#[derive(Debug)]
pub enum IPFSErrorKind {
    Success,
    InvalidHandle,
    Utf8Error,
    InvalidMethod,
    InvalidParameter,
    InvalidEncoding,
    RequestError,
    RuntimeError,
    TooManySessions,
    InvalidDriver,
    PermissionDeny,
}

impl std::error::Error for IPFSErrorKind {}

impl std::fmt::Display for IPFSErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Success => write!(f, "Success"),
            Self::InvalidHandle => write!(f, "Invalid Handle"),
            Self::Utf8Error => write!(f, "UTF-8 Error"),
            Self::InvalidMethod => write!(f, "Invalid Method"),
            Self::InvalidParameter => write!(f, "Invalid Parameter"),
            Self::InvalidEncoding => write!(f, "Invalid Encoding"),
            Self::RequestError => write!(f, "Request Error"),
            Self::RuntimeError => write!(f, "Runtime Error"),
            Self::TooManySessions => write!(f, "Too many sessions"),
            Self::InvalidDriver => write!(f, "Invalid Driver"),
            Self::PermissionDeny => write!(f, "Permission Deny"),
        }
    }
}

impl From<u32> for IPFSErrorKind {
    fn from(i: u32) -> IPFSErrorKind {
        match i {
            0 => IPFSErrorKind::Success,
            1 => IPFSErrorKind::InvalidHandle,
            2 => IPFSErrorKind::Utf8Error,
            3 => IPFSErrorKind::InvalidMethod,
            4 => IPFSErrorKind::InvalidParameter,
            5 => IPFSErrorKind::InvalidEncoding,
            6 => IPFSErrorKind::RequestError,
            7 => IPFSErrorKind::RuntimeError,
            8 => IPFSErrorKind::TooManySessions,
            9 => IPFSErrorKind::InvalidDriver,
            10 => IPFSErrorKind::PermissionDeny,
            _ => IPFSErrorKind::RuntimeError,
        }
    }
}

#[derive(Debug)]
pub enum AWSS3ErrorKind {
    Success,
    InvalidHandle,
    Utf8Error,
    InvalidMethod,
    InvalidParameter,
    InvalidEncoding,
    CredentialsError,
    RegionError,
    RequestError,
    RuntimeError,
    TooManySessions,
    InvalidDriver,
    PermissionDeny,
}

impl std::fmt::Display for AWSS3ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Success => write!(f, "Success"),
            Self::InvalidHandle => write!(f, "Invalid Handle"),
            Self::Utf8Error => write!(f, "UTF-8 Error"),
            Self::InvalidMethod => write!(f, "Invalid Method"),
            Self::InvalidParameter => write!(f, "Invalid Parameter"),
            Self::InvalidEncoding => write!(f, "Invalid Encoding"),
            Self::CredentialsError => write!(f, "Credentials Error"),
            Self::RegionError => write!(f, "Region Error"),
            Self::RequestError => write!(f, "Request Error"),
            Self::RuntimeError => write!(f, "Runtime Error"),
            Self::TooManySessions => write!(f, "Too many sessions"),
            Self::InvalidDriver => write!(f, "Invalid Driver"),
            Self::PermissionDeny => write!(f, "Permission Deny"),
        }
    }
}

impl From<u32> for AWSS3ErrorKind {
    fn from(i: u32) -> AWSS3ErrorKind {
        match i {
            0 => AWSS3ErrorKind::Success,
            1 => AWSS3ErrorKind::InvalidHandle,
            2 => AWSS3ErrorKind::Utf8Error,
            3 => AWSS3ErrorKind::InvalidMethod,
            4 => AWSS3ErrorKind::InvalidParameter,
            5 => AWSS3ErrorKind::InvalidEncoding,
            6 => AWSS3ErrorKind::CredentialsError,
            7 => AWSS3ErrorKind::RegionError,
            8 => AWSS3ErrorKind::RequestError,
            9 => AWSS3ErrorKind::TooManySessions,
            10 => AWSS3ErrorKind::InvalidDriver,
            11 => AWSS3ErrorKind::PermissionDeny,
            _ => AWSS3ErrorKind::RuntimeError,
        }
    }
}

impl std::error::Error for AWSS3ErrorKind {}
