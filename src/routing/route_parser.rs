use crate::routing::routes::Route;
use core::error;
use std::fmt;

#[derive(Debug, Clone)]
struct ParseError;
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}
impl error::Error for ParseError {}

#[derive(Debug, Clone)]
struct UnsupportedMethodError;
impl fmt::Display for UnsupportedMethodError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}
impl error::Error for UnsupportedMethodError {}

pub enum HTTPMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
}

enum HTTPVersion {
    HTTP1dot1,
}

pub struct RequestLine {
    pub method: HTTPMethod,
    pub route: Route,

    #[allow(unused)]
    version: HTTPVersion,
}

pub fn parse_request_line(request_line: String) -> Result<RequestLine, Box<dyn error::Error>> {
    let parts: Vec<&str> = request_line.split(' ').collect();

    if parts.len() != 3 {
        return Err(ParseError.into());
    }

    let method;
    match parts[0] {
        "GET" => method = HTTPMethod::GET,
        "PUT" => method = HTTPMethod::PUT,
        "POST" => method = HTTPMethod::POST,
        "PATCH" => method = HTTPMethod::PATCH,
        "DELETE" => method = HTTPMethod::DELETE,
        _ => return Err(UnsupportedMethodError.into()),
    }

    let route = Route::from_string(parts[1].to_string());
    let route = match route {
        Err(_) => return Err(ParseError.into()),
        Ok(r) => r,
    };

    let version = match parts[2] {
        "HTTP/1.1" => HTTPVersion::HTTP1dot1,
        _ => return Err(ParseError.into()),
    };

    return Ok(RequestLine {
        method,
        route,
        version,
    });
}
