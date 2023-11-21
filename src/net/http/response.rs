use super::HTTPHeader;

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct HTTPResponse {
    pub version: String,
    pub status_code: usize,
    pub status_msg: String,
    pub headers: Vec<HTTPHeader>,
    pub body: Vec<u8>
}

impl HTTPResponse {
    pub fn new(version: &str, status_code: usize, status_msg: &str, body: &[u8]) -> Self{
        Self {
            version: version.to_string(),
            status_code,
            status_msg: status_msg.to_string(),
            headers: vec![],
            body: body.to_vec()
        }
    }

    pub fn add_header(&mut self, header: &HTTPHeader) {
        self.headers.push(header.clone());
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let status_line = format!("{} {} {}", self.version, self.status_code, self.status_msg);
        let mut headers = vec![];

        self.headers.iter().for_each(|header| {
            headers.push(format!("{}: {}", header.0, header.1));
        });

        let mut response_string = status_line;
        headers.iter().for_each(|header| {
            response_string = format!("{}\r\n{}", response_string, header);
        });

        let mut response_bytes = response_string.as_bytes().to_vec();
        response_bytes.extend_from_slice(&self.body);
        response_bytes
    }
}