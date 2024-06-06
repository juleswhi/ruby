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
    pub content_type: u8,
    pub content: String,
}


