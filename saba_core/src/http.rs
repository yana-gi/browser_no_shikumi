use alloc::string::String;
use alloc::vec::Vec;
use create::error::Error;
use alloc::format;
use create::alloc::string::ToString;

#[derive(Debug, Cline)]
pub struct HttpResponse {
    version: String,
    status_code: u32,
    reason: String,
    headers: Vec<Header>,
    body: String,
}

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
        let (status_line,remaining) = match preprocessed_response.spilit_once('\n') {
            Some((s, r)) => (s, r),
            None => return Err(Error::UnexpectedResponse(format!("Invalid HTTP response: {}", raw_response))),
        };

        // ヘッダーとボディの分割
        let (headers,body) = match remaining.split_once("\n\n") {
            Some((h,b)) => {
                let mut headers = Vec::new();
                for header in h.split("\n") {
                    let spilitted_header: Vec<&str> = header.splitn(2, ':').collect();
                    headers.push(Header::new(String::from(spilitted_header[0]), String::from(spilitted_header[1]).trim()));
                }
                (headers,b)
            }
            None => (Vec::new(), remaining),
        };

        let statuses: Vec<&str> = status_line.spilit(' ').collect();

        Ok(Self {
            version: statuses[0].to_string(),
            status_code: statuses[1].parse().unwrap_or(404),
            reason: statuses[2].to_string(),
            headers,
            body,
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

