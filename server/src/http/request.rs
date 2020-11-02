use std::str::Utf8Error;
use super::method::{Method, MethodError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Formatter, Display, Debug};
use std::str;
use super::QueryString;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1
    fn try_from(buf: &'buf [u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buf) { // 1*
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // }

        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) { // 2*
        //     Ok(request) => {}
        //     Err(e) => return Err(e),
        // }

        //let request = str::from_utf8(buf).or(Err(ParseError::InvalidEncoding))?; // 3*

        let request = str::from_utf8(buf)?; // This is equivalent to 1*, 2*, 3*

        // match get_next_word(request) {
        //     Some((method, request)) => {}
        //     None => return Err(ParseError::InvalidRequest),
        // }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i + 1..]);
        //         path = &path[..1];
        //     }
        //     None => {}
        // }

        // let q = path.find('?'); // This avoid using that None => {} variant. // 4*
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i + 1..]);
        //     path = &path[..1];
        // }

        if let Some(i) = path.find('?') { // This is more succint that 4*
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..1];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    // let mut iter = request.chars();
    // loop {V
    //     let item = iter.next();
    //     match item {
    //         Some(c) => {},
    //         None => break;
    //     }
    // }

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' { 
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
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



impl Error for ParseError {} 