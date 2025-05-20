use std::error::Error;
use std::convert::TryFrom;
use std::str::{self, Utf8Error};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use super::method::MethodError;





pub struct Request {
    pub path: String,
    query_string: Option<String>,
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Request, Self::Error> {
        // Convert the byte slice to a String
        let req_res = String::from_utf8(buf.to_vec());

        match req_res {
            Ok(req) => {
                println!("{}", req);
                // Here, you need to parse `req` and extract path/query_string, etc.
                Ok(Request {
                    path: req, // You need to extract a valid path from `req`
                    query_string: None, // For now, no query string logic
                })
            },
            Err(e) => {
                println!("Error in parsing: {}", e);
                Err(ParseError::InvalidEncoding) // Return your custom error
            },
        }
    }
}

pub enum ParseError {
    InvalidRequest,
    InvalidProtocall,
    InvalidMethod,
    InvalidEncoding,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidRequest => "Invalid request!",
            ParseError::InvalidMethod => "Invalid method",
            ParseError::InvalidProtocall => "Invalid protocall",
            ParseError::InvalidEncoding => "Invalid encoding",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<MethodError> for ParseError {
    fn from(_:MethodError) -> Self {
        ParseError::InvalidMethod
            }
}

// Removed the MethodError since it's not defined and not used

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        ParseError::InvalidEncoding
    }
}

impl Error for ParseError {}
