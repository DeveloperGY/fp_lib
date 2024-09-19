use std::collections::HashMap;

#[derive(Debug)]
pub enum HttpResponseBuildError {
    MissingVersion,
    MissingStatusCode,
    MissingStatusMessage,
    MissingBody,
}

pub struct HttpResponseBuilder {
    version: Option<String>,
    status_code: Option<u16>,
    status_message: Option<String>,
    headers: Option<HashMap<String, String>>,
    body: Option<Box<[u8]>>,
}

impl HttpResponseBuilder {
    pub fn new() -> Self {
        Self {
            version: None,
            status_code: None,
            status_message: None,
            headers: Some(HashMap::new()),
            body: None,
        }
    }
    pub fn set_version(&mut self, version: String) -> &mut Self {
        self.version = Some(version);
        self
    }

    pub fn set_status_code(&mut self, status_code: u16) -> &mut Self {
        self.status_code = Some(status_code);
        self
    }

    pub fn set_status_message(&mut self, status_message: String) -> &mut Self {
        self.status_message = Some(status_message);
        self
    }

    pub fn set_header(&mut self, key: String, value: String) -> &mut Self {
        self.headers.as_mut().unwrap().insert(key, value);
        self
    }

    /// This will clear all currently set headers
    pub fn set_headers(&mut self, headers: HashMap<String, String>) -> &mut Self {
        self.headers = Some(headers);
        self
    }

    pub fn set_body(&mut self, body: Box<[u8]>) -> &mut Self {
        self.body = Some(body);
        self
    }

    pub fn build(&mut self) -> Result<HttpResponse, HttpResponseBuildError> {
        if self.version.is_none() {
            return Err(HttpResponseBuildError::MissingVersion);
        }

        if self.status_code.is_none() {
            return Err(HttpResponseBuildError::MissingStatusCode);
        }

        if self.status_message.is_none() {
            return Err(HttpResponseBuildError::MissingStatusMessage);
        }

        if self.body.is_none() {
            return Err(HttpResponseBuildError::MissingBody);
        }

        let res = Ok(HttpResponse::new(
            self.version.take().unwrap(),
            self.status_code.take().unwrap(),
            self.status_message.take().unwrap(),
            self.headers.take().unwrap(),
            self.body.take().unwrap(),
        ));

        self.headers = Some(HashMap::new());

        res
    }
}

/// HTTP 1.x Response
pub struct HttpResponse {
    version: String,
    status_code: u16,
    status_message: String,
    headers: HashMap<String, String>,
    body: Box<[u8]>,
}

impl HttpResponse {
    pub fn new(
        version: String,
        status_code: u16,
        status_message: String,
        headers: HashMap<String, String>,
        body: Box<[u8]>,
    ) -> Self {
        Self {
            version,
            status_code,
            status_message,
            headers,
            body,
        }
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    pub fn get_status_code(&self) -> u16 {
        self.status_code
    }

    pub fn set_status_code(&mut self, status_code: u16) {
        self.status_code = status_code;
    }

    pub fn get_status_message(&self) -> &str {
        &self.status_message
    }

    pub fn set_status_message(&mut self, status_message: String) {
        self.status_message = status_message;
    }

    pub fn get_header(&self, key: &String) -> Option<&String> {
        self.headers.get(key)
    }

    pub fn get_headers(&self) -> &HashMap<String, String> {
        &self.headers
    }

    pub fn set_header(&mut self, key: String, val: String) {
        self.headers.insert(key, val);
    }

    pub fn get_body(&self) -> &[u8] {
        &self.body
    }

    pub fn set_body(&mut self, body: Box<[u8]>) {
        self.body = body;
    }

    pub fn builder() -> HttpResponseBuilder {
        HttpResponseBuilder::new()
    }

    pub fn as_bytes(&self) -> Box<[u8]> {
        let mut bytes = Vec::new();

        // Encode Status Line
        bytes.extend_from_slice(self.version.as_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(self.status_code.to_string().as_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(self.status_message.as_bytes());
        bytes.extend_from_slice(b"\r\n");

        // Encode Headers
        for (key, val) in &self.headers {
            bytes.extend_from_slice(key.as_bytes());
            bytes.extend_from_slice(b": ");
            bytes.extend_from_slice(val.as_bytes());
            bytes.extend_from_slice(b"\r\n");
        }
        bytes.extend_from_slice(b"\r\n");

        // Encode body
        bytes.extend_from_slice(&self.body);

        bytes.into_boxed_slice()
    }

    pub fn into_bytes(self) -> Box<[u8]> {
        let mut bytes = Vec::new();

        // Encode Status Line
        bytes.extend_from_slice(&self.version.into_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(&self.status_code.to_string().into_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(&self.status_message.into_bytes());
        bytes.extend_from_slice(b"\r\n");

        // Encode Headers
        for (key, val) in self.headers {
            bytes.extend_from_slice(&key.into_bytes());
            bytes.extend_from_slice(b": ");
            bytes.extend_from_slice(&val.into_bytes());
            bytes.extend_from_slice(b"\r\n");
        }
        bytes.extend_from_slice(b"\r\n");

        // Encode body
        bytes.extend_from_slice(&self.body);

        bytes.into_boxed_slice()
    }
}
