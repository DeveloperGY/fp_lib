use std::{
    collections::{HashMap, VecDeque},
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::TcpStream,
};

use super::HttpRequest;
use super::HttpResponse;
pub use crate::io::{IntoSplit, SplitMut};

pub struct HttpReceiverMut<'http, 'tcp: 'http> {
    rx: &'http mut BufReader<&'tcp TcpStream>,
}

impl<'http, 'tcp: 'http> HttpReceiverMut<'http, 'tcp> {
    pub(crate) fn new(rx: &'http mut BufReader<&'tcp TcpStream>) -> Self {
        Self { rx }
    }

    pub fn recv_request(&mut self) -> std::io::Result<HttpRequest> {
        receive_http_request(self.rx)
    }

    pub fn recv_response(&mut self) -> std::io::Result<HttpResponse> {
        receive_http_response(self.rx)
    }
}

pub struct HttpTransmitterMut<'http, 'tcp: 'http> {
    tx: &'http mut BufWriter<&'tcp std::net::TcpStream>,
}

impl<'http, 'tcp: 'http> HttpTransmitterMut<'http, 'tcp> {
    pub(crate) fn new(tx: &'http mut BufWriter<&'tcp TcpStream>) -> Self {
        Self { tx }
    }

    pub fn send_request(&mut self, request: &HttpRequest) -> Result<(), std::io::Error> {
        send_http_request(self.tx, request)
    }

    pub fn send_response(&mut self, response: &HttpResponse) -> Result<(), std::io::Error> {
        send_http_response(self.tx, response)
    }
}

pub struct HttpReceiver<'tcp> {
    rx: BufReader<&'tcp TcpStream>,
}

impl<'tcp> HttpReceiver<'tcp> {
    pub(crate) fn new(rx: BufReader<&'tcp TcpStream>) -> Self {
        Self { rx }
    }

    pub fn recv_request(&mut self) -> std::io::Result<HttpRequest> {
        receive_http_request(&mut self.rx)
    }

    pub fn recv_response(&mut self) -> Result<HttpResponse, std::io::Error> {
        receive_http_response(&mut self.rx)
    }
}

pub struct HttpTransmitter<'tcp> {
    tx: BufWriter<&'tcp std::net::TcpStream>,
}

impl<'tcp> HttpTransmitter<'tcp> {
    pub(crate) fn new(tx: BufWriter<&'tcp TcpStream>) -> Self {
        Self { tx }
    }

    pub fn send_request(&mut self, request: &HttpRequest) -> Result<(), std::io::Error> {
        send_http_request(&mut self.tx, request)
    }

    pub fn send_response(&mut self, response: &HttpResponse) -> Result<(), std::io::Error> {
        send_http_response(&mut self.tx, response)
    }
}

pub struct HttpStream<'tcp> {
    rx: BufReader<&'tcp TcpStream>,
    tx: BufWriter<&'tcp TcpStream>,
}

impl<'tcp> HttpStream<'tcp> {
    /// Using the TcpStream passed into this function after it is called can lead to data loss and
    /// as such is inadvisable, just like when using the underlying reader of a BufReader
    pub fn new(
        stream: &'tcp TcpStream, /*, read_timeout: std::time::Duration*/
    ) -> std::io::Result<Self> {
        //debug_assert!(!read_timeout.is_zero());
        //let _ = stream.set_read_timeout(Some(read_timeout));
        stream.set_nonblocking(true)?;
        let rx = BufReader::new(stream);
        let tx = BufWriter::new(stream);

        Ok(Self { rx, tx })
    }

    pub fn send_request(&mut self, request: &HttpRequest) -> Result<(), std::io::Error> {
        send_http_request(&mut self.tx, request)
    }

    pub fn recv_request(&mut self) -> std::io::Result<HttpRequest> {
        receive_http_request(&mut self.rx)
    }

    pub fn send_response(&mut self, response: &HttpResponse) -> Result<(), std::io::Error> {
        send_http_response(&mut self.tx, response)
    }

    pub fn recv_response(&mut self) -> std::io::Result<HttpResponse> {
        receive_http_response(&mut self.rx)
    }
}

impl<'http, 'tcp: 'http>
    SplitMut<'http, HttpReceiverMut<'http, 'tcp>, HttpTransmitterMut<'http, 'tcp>>
    for HttpStream<'tcp>
{
    fn split_mut(
        &'http mut self,
    ) -> (
        HttpReceiverMut<'http, 'tcp>,
        HttpTransmitterMut<'http, 'tcp>,
    ) {
        (
            HttpReceiverMut::new(&mut self.rx),
            HttpTransmitterMut::new(&mut self.tx),
        )
    }
}

impl<'tcp> IntoSplit<HttpReceiver<'tcp>, HttpTransmitter<'tcp>> for HttpStream<'tcp> {
    fn into_split(self) -> (HttpReceiver<'tcp>, HttpTransmitter<'tcp>) {
        (HttpReceiver::new(self.rx), HttpTransmitter::new(self.tx))
    }
}

fn receive_http_request(rx: &mut BufReader<&TcpStream>) -> std::io::Result<HttpRequest> {
    // Get Request line
    let mut request_line_buf = String::new();
    'reading_request_line: loop {
        match rx.read_line(&mut request_line_buf) {
            Err(err) => match err.kind() {
                std::io::ErrorKind::WouldBlock => continue,
                _ => {
                    return Err(err);
                }
            },
            _ => break 'reading_request_line,
        };
    }

    // Get Headers
    let mut header_strings = Vec::new();

    'reading_headers: loop {
        let mut buf = String::new();
        match rx.read_line(&mut buf) {
            Err(err) => match err.kind() {
                std::io::ErrorKind::WouldBlock => continue,
                _ => {
                    return Err(err);
                }
            },
            _ => (),
        };

        let line = buf.trim();

        if line.len() == 0 {
            break 'reading_headers;
        }

        header_strings.push(line.to_string());
    }

    // Get Body
    let mut bytes = Vec::new();

    const BUFFER_SIZE: usize = 512;
    let mut buffer = [0_u8; BUFFER_SIZE];
    'reading_body: loop {
        let bytes_read = match rx.read(&mut buffer) {
            Ok(val) => val,
            Err(err) => match err.kind() {
                std::io::ErrorKind::WouldBlock => 0,
                _ => {
                    return Err(err);
                }
            },
        };

        bytes.extend_from_slice(&buffer[..bytes_read]);

        if bytes_read < BUFFER_SIZE {
            break 'reading_body;
        }
    }

    // Process Request Line
    let request_line = request_line_buf.trim().to_string();
    let mut words: Vec<_> = request_line.split(' ').collect();

    if words.len() != 3 {
        Err(std::io::Error::other("Invalid Request Line"))?;
    }

    let version = words.pop().unwrap().to_string();
    let url = words.pop().unwrap().to_string();
    let method = words.pop().unwrap().to_string();

    // Process Headers
    let headers: HashMap<String, String> = header_strings
        .into_iter()
        .map(|line| {
            line.split_once(':')
                .map(|(key, val)| (key.to_string(), val.to_string()))
        })
        .filter(|val| val.is_some())
        .map(|val| val.unwrap())
        .collect();

    // Process Body
    let body = bytes.into_boxed_slice();

    // Build Request
    let mut builder = HttpRequest::builder();

    Ok(builder
        .set_method(method)
        .set_url(url)
        .set_version(version)
        .set_body(body)
        .set_headers(headers)
        .build()
        .unwrap())
}

fn send_http_request(tx: &mut BufWriter<&TcpStream>, request: &HttpRequest) -> std::io::Result<()> {
    tx.write_all(&request.as_bytes())?;
    tx.flush()
}

fn receive_http_response(rx: &mut BufReader<&TcpStream>) -> std::io::Result<HttpResponse> {
    // Get Status line
    let mut status_line_buf = String::new();

    'reading_status_line: loop {
        match rx.read_line(&mut status_line_buf) {
            Err(err) => match err.kind() {
                std::io::ErrorKind::WouldBlock => continue,
                _ => {
                    return Err(err);
                }
            },
            _ => break 'reading_status_line,
        };
    }

    // Get Headers
    let mut header_strings = Vec::new();

    'reading_headers: loop {
        let mut buf = String::new();

        match rx.read_line(&mut buf) {
            Err(err) => match err.kind() {
                std::io::ErrorKind::WouldBlock => continue,
                _ => {
                    return Err(err);
                }
            },
            _ => (),
        };

        let line = buf.trim();

        if line.len() == 0 {
            break 'reading_headers;
        }

        header_strings.push(line.to_string());
    }

    // Get Body
    let mut bytes = Vec::new();

    const BUFFER_SIZE: usize = 512;
    let mut buffer = [0_u8; BUFFER_SIZE];
    'reading_body: loop {
        let bytes_read = match rx.read(&mut buffer) {
            Ok(val) => val,
            Err(err) => match err.kind() {
                std::io::ErrorKind::WouldBlock => 0,
                _ => {
                    return Err(err);
                }
            },
        };

        bytes.extend_from_slice(&buffer[..bytes_read]);

        if bytes_read < BUFFER_SIZE {
            break 'reading_body;
        }
    }

    // Process Status Line
    let status_line = status_line_buf.trim().to_string();
    let mut words: VecDeque<_> = status_line.split(' ').collect();

    if words.len() < 3 {
        Err(std::io::Error::other("Invalid Status Line"))?;
    }

    let version = words.pop_front().unwrap().to_string();
    let status_code = words
        .pop_front()
        .unwrap()
        .to_string()
        .parse::<u16>()
        .map_err(|_| std::io::Error::other("Failed to parse status code"))?;
    let status_message = words.into_iter().collect::<Vec<_>>().join(" ");

    // Process Headers
    let headers: HashMap<String, String> = header_strings
        .into_iter()
        .map(|line| {
            line.split_once(':')
                .map(|(key, val)| (key.to_string(), val.to_string()))
        })
        .filter(|val| val.is_some())
        .map(|val| val.unwrap())
        .collect();

    // Process Body
    let body = bytes.into_boxed_slice();

    // Build Request
    let mut builder = HttpResponse::builder();

    Ok(builder
        .set_version(version)
        .set_status_code(status_code)
        .set_status_message(status_message)
        .set_headers(headers)
        .set_body(body)
        .build()
        .unwrap())
}

fn send_http_response(
    tx: &mut BufWriter<&TcpStream>,
    response: &HttpResponse,
) -> std::io::Result<()> {
    tx.write_all(&response.as_bytes())?;
    tx.flush()
}
