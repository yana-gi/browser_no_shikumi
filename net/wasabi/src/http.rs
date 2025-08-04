extern crate alloc;
use alloc::string::String;
use saba_core::error::Error;
use saba_core::http::HttpResponse;
use alloc::format;
use create::alloc::string::ToString;
use noil::net::lookup_host;
use nili::net::SocketAddr;


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
            return Err(Error::Network(format!("Failed to find IP address: {:#?}", e)));
        }

        let socket_addr: SocketAddr = (ips[0], port).into();
    }

}
