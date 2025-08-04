extern crate alloc;
use alloc::string::String;
use saba_core::error::Error;
use saba_core::http::HttpResponse;
use alloc::format;
use alloc::string::ToString;
use noli::net::lookup_host;
use noli::net::SocketAddr;
use noli::net::TcpStream;
use alloc::vec::Vec;


pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }
    pub fn get(&self, host:String, port:u16,path:String) -> Result<HttpResponse, Error> {
        let ips = match lookup_host(&host) {
            Ok(ips) => ips,
            Err(e) => return Err(Error::Network(format!("Failed to lookup find IP address: {:#?}", e))),
        };

        if ips.len() < 1{
            return Err(Error::Network(format!("Failed to find IP address for host: {:?}", host)));
        }

        let socket_addr: SocketAddr = (ips[0], port).into();
        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(stream) => stream,
            Err(_) => return Err(Error::Network("Failed to connect to TCP stream".to_string())),
        };

        let mut request = String::from("GET ");
        request.push_str(&path);
        request.push_str(" HTTP/1.1\r\n");

        // ヘッダーを追加
        request.push_str("Host: ");
        request.push_str(&host);
        request.push_str("\n");
        request.push_str("Accept: text/html\n");
        request.push_str("\n");
        
        // リクエストを送信
        let _bytes_written = match stream.write(request.as_bytes()) {
            Ok(bytes) => bytes,
            Err(_) => {
                return Err(Error::Network("Failed to write request to TCP stream".to_string()));
            }
        };
        // レスポンスの受信
        let mut received = Vec::new();
        loop {
            // httpレスポンスを格納するためのバッファ
            let mut buf = [0u8; 4096];
            let bytes_read = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(Error::Network("Failed to read response from TCP stream".to_string()));
                }
            };
            // ストリームから読み込むバイトがなくなるまでroopを回す
            if bytes_read == 0 {
                break;
            }
            received.extend_from_slice(&buf[..bytes_read]);
        }
        match core::str::from_utf8(&received) {
            Ok(response) => HttpResponse::new(response.to_string()),
            Err(e) => return Err(Error::Network(format!("Failed to parse response from TCP stream: {:#?}", e))),
        }
    }
}
