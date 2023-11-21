use super::HTTPHeader;
use std::net::TcpStream;
use std::io::Read;

#[derive(Debug, Clone)]
pub struct HTTPRequest {
    pub method: String,
    pub uri: String,
    pub version: String,
    pub headers: Vec<HTTPHeader>,
    pub body: Vec<u8>
}

impl HTTPRequest {
    pub fn new(method: &str, uri: &str, version: &str, headers: &[HTTPHeader], body: &[u8]) -> Self {
        Self {
            method: method.to_string(),
            uri: uri.to_string(),
            version: version.to_string(),
            headers: headers.to_vec(),
            body: body.to_vec()
        }
    }
}

impl HTTPRequest {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() == 0 {
            return Err("Invalid byte count!".into());
        }   

        let body_index = HTTPRequest::find_body_index(bytes)?;
        let body = HTTPRequest::get_body(bytes, body_index);

        let meta = String::from_utf8(bytes[..body_index].to_vec()).map_err(|_| String::from("Invalid UTF-8!"))?;

        let (status_string, header_strings) = HTTPRequest::split_meta(&meta)?;
        let (method, uri, version) = HTTPRequest::split_status(&status_string)?;
        let headers = HTTPRequest::parse_headers(header_strings.as_slice());

        Ok(Self::new(&method, &uri, &version, &headers, &body))
    }
    
    pub fn from_stream(stream: &mut TcpStream) -> Result<Self, String> {
        let bytes = HTTPRequest::read_stream(stream)?;
        HTTPRequest::from_bytes(bytes.as_slice())
    }

    /**
     * Reads an http request from a tcp stream
     */
    fn read_stream(stream: &mut TcpStream) -> Result<Vec<u8>, String> {
        let mut request_bytes = vec![];
        
        const BUFFER_SIZE: usize = 1024;
        let mut buffer = [0; 1024];

        loop {
            let bytes_read = stream.read(&mut buffer).map_err(|_| {String::from("Failed to read tcp stream!")})?;

            request_bytes.extend_from_slice(&buffer[..bytes_read]);

            if bytes_read < BUFFER_SIZE {
                break;
            }
        }

        Ok(request_bytes)
    }

    /**
     * Returns the index of the start of the body of an http request or an error if it doesnt exist 
     */
    fn find_body_index(bytes: &[u8]) -> Result<usize, String> {
        let windows = bytes.windows(4).enumerate().collect::<Vec<_>>();
        let mut body_index = 0;

        for (index, bytes) in windows {
            if bytes == b"\r\n\r\n" {
                body_index = index + 4;
                break;
            }
        }

        if body_index != 0 {
            Ok(body_index)
        }
        else {
            Err("Failed to find body index!".into())
        }
    }

    fn get_body(bytes: &[u8], index: usize) -> Vec<u8> {
        if index == bytes.len() {vec![]} else {bytes[index..].to_vec()}
    }

    /**
     * Retuns [`(status_line: String, headers: Vec<String>)`]
     */
    fn split_meta(meta: &str) -> Result<(String, Vec<String>), String> {
        if meta.trim().lines().count() < 1 {
            return Err("Invalid meta format!".into());
        }

        let mut status_line = String::new();
        let mut headers = vec![];

        meta.lines().enumerate().for_each(|(index, value)| {
            match index {
                0 => status_line = value.to_string(),
                _ => headers.push(value.to_string())
            };
        });

        Ok((status_line, headers))
    }

    /**
     * returns [`(method: String, uri: String, version: String)`]
     */
    fn split_status(status_line: &str) -> Result<(String, String, String), String> {
        if status_line.trim().split_whitespace().count() != 3 {
            return Err("Invalid status line!".into());
        }

        let status_words = status_line.trim().split_whitespace().collect::<Vec<_>>();
        let method = status_words[0].to_string();
        let uri = status_words[1].to_string();
        let version = status_words[2].to_string();

        Ok((method, uri, version))
    }

    fn parse_headers(header_strings: &[String]) -> Vec<HTTPHeader> {
        let mut headers = vec![];

        header_strings.iter().for_each(|header| {
            if let Some((key, value)) = header.split_once(": ") {
                headers.push(HTTPHeader(key.to_string(), value.to_string()));
            }
        });
        
        headers
    }
}