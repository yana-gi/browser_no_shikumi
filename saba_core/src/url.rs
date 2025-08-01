use alloc::string::String;
use alloc::vec::Vec;
use alloc::string::ToString;

impl Url {
    pub fn host(&self)-> String {
        self.host.clone()
    }

    pub fn port(&self)-> String {
        self.port.clone()
    }

    pub fn path(&self)-> String {
        self.path.clone()
    }

    pub fn searchpart(&self)-> String {
        self.searchpart.clone()
    }

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

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();
        
        Ok(self.clone())
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

    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
        .url
        .trim_start_matches("http://")
        .splitn(2, '/')
        .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][index + 1..].to_string()
        } else {
            "80".to_string()
        }
    }

    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self
        .url
        .trim_start_matches("http://")
        .splitn(2, '/')
        .collect();

        // パスが含まれていない場合は、空文字列を戻り値として返す
        if url_parts.len() < 2 {
            return "".to_string();
        }

        // パスとクエリパラメータを分割
        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, '?').collect();

        // パスを戻り値として返す
        path_and_searchpart[0].to_string()
    }

    fn extract_searchpart(&self) -> String {
        let url_parts: Vec<&str> = self
        .url
        .trim_start_matches("http://")
        .splitn(2, '/')
        .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, '?').collect();

        if path_and_searchpart.len() < 2 {
            return "".to_string();
        } else {
            path_and_searchpart[1].to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_host() {
        let url = "http://example.com".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected,Url::new(url).parse());
    }

    #[test]
    fn test_url_port() {
        let url = "http://example.com:8888".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected,Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port_path() {
        let url = "http://example.com:8888/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected,Url::new(url).parse());
    }

    #[test]
    fn test_url_host_path() {
        let url = "http://example.com/index.html".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "index.html".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(expected,Url::new(url).parse());
    }

    #[test]
    fn test_url_host_port_path_searchquery() {
        let url = "http://example.com:8888/index.html?a=123&b=456".to_string();
        let expected = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8888".to_string(),
            path: "index.html".to_string(),
            searchpart: "a=123&b=456".to_string(),
        });
        assert_eq!(expected,Url::new(url).parse());
    }

    #[test]
    fn test_no_scheme() {
        let url = "example.com".to_string();
        let expected = Err("Only HTTP scheme is supported".to_string());
        assert_eq!(expected,Url::new(url).parse());
    }

    #[test]
    fn test_unsupported_scheme() {
        let url = "https://example.com:8888/index.html".to_string();
        let expected = Err("Only HTTP scheme is supported".to_string());
        assert_eq!(expected,Url::new(url).parse());
    }
}
