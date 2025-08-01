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

    // ホスト名を戻り値として返す
    fn extract_host(&self) -> String {
        let url_parts: Vec<&str> = self
        .url
        // 先頭のhttp://を削除
        .trim_start_matches("http://")
        // 先頭のスラッシュまでを分割
        .splitn(2, '/')
        // ベクタに格納
        .collect();

        // ポート番号が含まれている場合は、ポート番号より前の文字列を戻り値として返す
        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][..index].to_string()
        // ポート番号が含まれていない場合は、ホスト名を戻り値として返す
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
