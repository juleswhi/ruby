use crate::saph;
use crate::saph::SaphType;

use super::TCP_VERSION;

pub struct SaphRequest {
    pub version: u8,
    pub request_type: saph::RequestType,

    pub host: String,
    pub incr: u8,

    pub path_len: u16,
    pub path: String,

    pub content_len: u16,
    pub content_type: saph::ContentType,
    pub content: String,
}

impl SaphRequest {
    pub fn new(host: String) -> Self {
        SaphRequest {
            version: TCP_VERSION,
            request_type: saph::RequestType::GET,
            host,
            incr: 0,
            path_len: 1,
            path: "/".into(),
            content_len: 0,
            content_type: saph::ContentType::Plaintext,
            content: String::new(),
        }
    }

    pub fn to_bytes(self: &Self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.push(self.version);
        bytes.push(self.request_type.byte());

        let mut host_bytes: Vec<u8> = Vec::new();

        for st in self.host.split(".") {
            let num = st.parse::<u8>();
            host_bytes.push(num.unwrap_or(0));
        }

        bytes.append(&mut host_bytes);
        bytes.push(self.incr);

        bytes.extend_from_slice(&(self.path.len() as u16).to_be_bytes()[0..2]);
        bytes.extend_from_slice(&self.path.as_bytes());
        bytes.extend_from_slice(&(self.content.len() as u32).to_be_bytes()[0..4]);
        bytes.push(self.content_type.byte());
        bytes.extend_from_slice(&self.content.as_bytes());

        return bytes;
    }
}
