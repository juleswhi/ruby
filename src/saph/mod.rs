pub mod response;
pub mod request;

const TCP_VERSION: u8 = 1;

pub trait SaphType {
    fn byte(&self) -> u8;
}

pub enum RequestType {
    GET,
    GIVE,
    INCR,
}

impl RequestType {
    pub fn string(self: &Self) -> String {
        match self {
            Self::GET => "GET".into(),
            Self::GIVE => "GIVE".into(),
            Self::INCR => "INCR".into()
        }
    }
    pub fn from(n: u8) -> Option<Self> {
        match n {
            0 => Some(RequestType::GET),
            1 => Some(RequestType::GIVE),
            2 => Some(RequestType::INCR),
            _ => None,
        }
    }
}

impl SaphType for RequestType {
    fn byte(self: &Self) -> u8 {
        match self {
            RequestType::GET => 0,
            RequestType::GIVE => 1,
            RequestType::INCR => 2,
        }
    }
}

pub enum ContentType {
    Plaintext,
    CodeMarkdown,
    CodeLua,
}

impl ContentType {
    pub fn from(n: u8) -> Option<Self> {
        match n {
            0 => Some(ContentType::Plaintext),
            1 => Some(ContentType::CodeMarkdown),
            2 => Some(ContentType::CodeLua),
            _ => None,
        }
    }
}

impl SaphType for ContentType {
    fn byte(self: &Self) -> u8 {
        match self {
            ContentType::Plaintext => 0,
            ContentType::CodeMarkdown => 1,
            ContentType::CodeLua => 2,
        }
    }
}
