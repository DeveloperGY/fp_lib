use std::collections::HashMap;

#[derive(Debug)]
pub enum HttpRequestBuildError {
    MissingMethod,
    MissingUrl,
    MissingVersion,
    MissingBody,
}

pub struct HttpRequestBuilder {
    method: Option<String>,
    url: Option<String>,
    version: Option<String>,
    headers: Option<HashMap<String, String>>,
    body: Option<Box<[u8]>>,
}

impl HttpRequestBuilder {
    pub fn new() -> Self {
        Self {
            method: None,
            url: None,
            version: None,
            headers: Some(HashMap::new()),
            body: None,
        }
    }

    pub fn set_method(&mut self, method: String) -> &mut Self {
        self.method = Some(method);
        self
    }

    pub fn set_url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }

    pub fn set_version(&mut self, version: String) -> &mut Self {
        self.version = Some(version);
        self
    }

    pub fn set_body(&mut self, body: Box<[u8]>) -> &mut Self {
        self.body = Some(body);
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

    pub fn build(&mut self) -> Result<HttpRequest, HttpRequestBuildError> {
        if self.method.is_none() {
            return Err(HttpRequestBuildError::MissingMethod);
        }

        if self.url.is_none() {
            return Err(HttpRequestBuildError::MissingUrl);
        }

        if self.version.is_none() {
            return Err(HttpRequestBuildError::MissingVersion);
        }

        if self.body.is_none() {
            return Err(HttpRequestBuildError::MissingBody);
        }

        let req = Ok(HttpRequest::new(
            self.method.take().unwrap(),
            self.url.take().unwrap(),
            self.version.take().unwrap(),
            self.headers.take().unwrap(),
            self.body.take().unwrap(),
        ));

        self.headers = Some(HashMap::new());

        req
    }
}

/// HTTP 1.x Request
pub struct HttpRequest {
    method: String,
    url: String,
    version: String,
    headers: HashMap<String, String>,
    body: Box<[u8]>,
}

impl HttpRequest {
    pub fn new(
        method: String,
        url: String,
        version: String,
        headers: HashMap<String, String>,
        body: Box<[u8]>,
    ) -> Self {
        Self {
            method,
            url,
            version,
            headers,
            body,
        }
    }

    pub fn get_method(&self) -> &str {
        &self.method
    }

    pub fn set_method(&mut self, method: String) {
        self.method = method;
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn get_version(&self) -> &str {
        &self.version
    }

    pub fn set_version(&mut self, version: String) {
        self.version = version;
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

    pub fn builder() -> HttpRequestBuilder {
        HttpRequestBuilder::new()
    }

    pub fn as_bytes(&self) -> Box<[u8]> {
        let mut bytes = Vec::new();

        // Encode Request Line
        bytes.extend_from_slice(self.method.as_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(self.url.as_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(self.version.as_bytes());
        bytes.extend_from_slice(b"\r\n");

        // Encode headers
        for (key, val) in &self.headers {
            bytes.extend_from_slice(key.as_bytes());
            bytes.extend_from_slice(b": ");
            bytes.extend_from_slice(val.as_bytes());
            bytes.extend_from_slice(b"\r\n");
        }

        bytes.extend_from_slice(b"\r\n");

        // Encode Body
        bytes.extend_from_slice(&self.body);

        bytes.into_boxed_slice()
    }

    pub fn into_bytes(self) -> Box<[u8]> {
        let mut bytes = Vec::new();

        // Encode Request Line
        bytes.extend_from_slice(&self.method.into_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(&self.url.into_bytes());
        bytes.push(b' ');
        bytes.extend_from_slice(&self.version.into_bytes());
        bytes.extend_from_slice(b"\r\n");

        // Encode headers
        for (key, val) in self.headers {
            bytes.extend_from_slice(&key.into_bytes());
            bytes.extend_from_slice(b": ");
            bytes.extend_from_slice(&val.into_bytes());
            bytes.extend_from_slice(b"\r\n");
        }

        bytes.extend_from_slice(b"\r\n");

        // Encode Body
        bytes.extend_from_slice(&self.body);

        bytes.into_boxed_slice()
    }
}
