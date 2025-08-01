use alloc::string::String;
use alloc::vec::Vec;

impl Url {
    pub fn new(url: String)-> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    pub fn parse(&mut self)-> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP scheme is supported".to_string());
        }
    }

    fn is_http(&mut self)-> bool {
        if self.url.starts_with("http://") {
            return true;
        }
        false
    }

    fn extract_host(&self) -> String {
        let url_parts: Vec<&str> = self
        .url
        .trim_start_matches("http://")
        .splitn(2, '/')
        .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]

pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String
}
