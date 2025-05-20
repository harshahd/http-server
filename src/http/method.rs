use std::str::FromStr;

pub enum Method {
    GET,
    POST,
    OPTIONS,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    TRACE,
    CONNECT,
}

pub struct MethodError;

impl FromStr for Method {
    type Err=MethodError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "OPTIONS" => Ok(Self::OPTIONS),
            "PUT" => Ok(Self::PUT),
            "PATCH" => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            "TRACE" => Ok(Self::TRACE),
            "CONNECT" => Ok(Self::CONNECT),
            "HEAD" => Ok(Self::HEAD),
            _ => Err(MethodError),
        }
    }
}
