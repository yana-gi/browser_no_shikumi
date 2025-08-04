use alloc::string::String;
use alloc::vec::Vec;
use crate::error::Error;
use alloc::format;
use alloc::string::ToString;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    version: String,
    status_code: u32,
    reason: String,
    headers: Vec<Header>,
    body: String,
}

#[derive(Debug, Clone)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

impl HttpResponse {
    pub fn new(raw_response: String) -> Result<Self, Error>{
        // 文字列の前処理
        // キャリッジリターンと改行シーケンスを単一の改行にする
        let preprocessed_response = raw_response.trim_start().replace("\r\n", "\n");

        // ステータスラインの分割
        let (status_line,remaining) = match preprocessed_response.split_once('\n') {
            Some((s, r)) => (s, r),
            None => return Err(Error::UnexpectedResponse(format!("Invalid HTTP response: {}", raw_response))),
        };

        // ヘッダーとボディの分割
        let (headers,body) = match remaining.split_once("\n\n") {
            Some((h,b)) => {
                let mut headers = Vec::new();
                for header in h.split("\n") {
                    let split_header: Vec<&str> = header.splitn(2, ':').collect();
                    if split_header.len() == 2 {
                        headers.push(Header::new(String::from(split_header[0]), String::from(split_header[1]).trim().to_string()));
                    }
                }
                (headers,b)
            }
            None => (Vec::new(), remaining),
        };

        let statuses: Vec<&str> = status_line.split(' ').collect();

        Ok(Self {
            version: statuses[0].to_string(),
            status_code: statuses[1].parse().unwrap_or(404),
            reason: statuses[2].to_string(),
            headers,
            body: body.trim().to_string(),
        })
    }

    // ゲッタメソッド
    pub fn version(&self) -> String {
        self.version.clone()
    }

    pub fn status_code(&self) -> u32 {
        self.status_code
    }

    pub fn reason(&self) -> String {
        self.reason.clone()
    }
    
    pub fn headers(&self) -> Vec<Header> {
        self.headers.clone()
    }

    pub fn body(&self) -> String {
        self.body.clone()
    }

    pub fn header_value(&self, name: &str) -> Result<String, String> {
        for h in &self.headers {
            if h.name == name {
                return Ok(h.value.clone());
            }
        }
        Err(format!("Header {} not found", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_line_only() {
        let raw = "HTTP/1.1 200 OK\n".to_string();
        let res = HttpResponse::new(raw).expect("Failed to parse http response");
        assert_eq!(res.version(), "HTTP/1.1");
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.reason(), "OK");
        assert_eq!(res.headers().len(), 0);
    }

    #[test]
    fn test_one_header() {
        let raw = "HTTP/1.1 200 OK\nDate: Mon, 04 Aug 2025 00:00:00 GMT\n\n".to_string();
        let res = HttpResponse::new(raw).expect("Failed to parse http response");
        assert_eq!(res.version(), "HTTP/1.1");
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.reason(), "OK");
        assert_eq!(res.headers().len(), 1);

        assert_eq!(res.header_value("Date").unwrap(), "Mon, 04 Aug 2025 00:00:00 GMT");
    }

    #[test]
    fn test_two_headers_with_white_space() {
        let raw = "HTTP/1.1 200 OK\nDate: Mon, 04 Aug 2025 00:00:00 GMT\nContent-length:42\n\n".to_string();
        let res = HttpResponse::new(raw).expect("Failed to parse http response");
        assert_eq!(res.version(), "HTTP/1.1");
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.reason(), "OK");
        assert_eq!(res.headers().len(), 2);
        assert_eq!(res.header_value("Date").unwrap(), "Mon, 04 Aug 2025 00:00:00 GMT");
        assert_eq!(res.header_value("Content-length").unwrap(), "42".to_string());
    }

    #[test]
    fn test_body() {
        let raw = "HTTP/1.1 200 OK\n\nHello, World!".to_string();
        let res = HttpResponse::new(raw).expect("Failed to parse http response");
        assert_eq!(res.version(), "HTTP/1.1");
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.reason(), "OK");
        assert_eq!(res.headers().len(), 0);
        assert_eq!(res.body(), "Hello, World!".to_string());
    }

    #[test]
    fn test_invalid() {
        let raw = "Invalid HTTP response".to_string();

        assert!(HttpResponse::new(raw).is_err());
    }
}
