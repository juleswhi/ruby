use std::{io::Read, net::TcpStream};

use crate::saph;

pub struct SaphResponse {
    pub version: u8,
    pub response_type: saph::RequestType,

    pub status_code: u8,
    pub incr: u8,
    pub incr_iden: u32,
    pub date_len: u16,
    pub date: String,

    pub content_len: u32,
    pub content_type: saph::ContentType,
    pub content: String,
}

impl SaphResponse {
    pub fn from(stream: &mut TcpStream) -> Option<Self> {
        let mut res = SaphResponse {
            version: saph::TCP_VERSION,
            response_type: saph::RequestType::GET,
            status_code: 10,
            incr: 0,
            incr_iden: 0,
            date_len: 0,
            date: "".to_string(),
            content_len: 0,
            content_type: saph::ContentType::Plaintext,
            content: "".to_string(),
        };

        let mut version_buf: [u8; 1] = [0; 1];
        let _ = stream.read_exact(&mut version_buf);
        if version_buf[0] != saph::TCP_VERSION {
            return None;
        }

        res.version = version_buf[0];

        let mut response_type_buf: [u8; 1] = [0; 1];
        let _ = stream.read_exact(&mut response_type_buf);
        match saph::RequestType::from(response_type_buf[0]) {
            Some(v) => res.response_type = v,
            None => return None,
        }

        let mut status_code_buf: [u8; 1] = [0; 1];
        let _ = stream.read_exact(&mut status_code_buf);
        res.status_code = status_code_buf[0];

        let mut incr_buf: [u8; 1] = [0; 1];
        let _ = stream.read_exact(&mut incr_buf);
        res.incr = incr_buf[0];

        let mut incr_iden_buf: [u8; 4] = [0; 4];
        let _ = stream.read_exact(&mut incr_iden_buf);
        res.incr_iden = u32::from_be_bytes(incr_iden_buf);

        let mut date_len_buf: [u8; 2] = [0; 2];
        let _ = stream.read_exact(&mut date_len_buf);
        res.date_len = u16::from_be_bytes(date_len_buf);

        let mut date_buf = vec![0; res.date_len as usize];
        let _ = stream.read_exact(&mut date_buf);
        res.date = match String::from_utf8(date_buf) {
            Ok(v) => v,
            Err(_) => return None,
        };

        let mut content_len_buf: [u8; 4] = [0; 4];
        let _ = stream.read_exact(&mut content_len_buf);
        res.content_len = u32::from_be_bytes(content_len_buf);

        let mut content_type_buf: [u8;1] = [0;1];
        let _ = stream.read_exact(&mut content_type_buf);
        res.content_type = match saph::ContentType::from(content_type_buf[0]) {
            Some(v) => v,
            None => return None,
        };


        let mut content_buf = vec![0; res.content_len as usize];
        let _ = stream.read_exact(&mut content_buf);
        res.content = match String::from_utf8(content_buf) {
            Ok(v) => v,
            Err(_) => return None,
        };

        Some(res)
    }
}
