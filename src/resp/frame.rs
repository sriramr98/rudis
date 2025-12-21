use std::fmt::format;

pub enum RespFrame {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(String),
    Array(Vec<RespFrame>),
    EmptyArray,
    Null,
    NullBulkString,
    NullArray
}

const CRLF: &str = "\r\n";

impl RespFrame {
    pub fn encode(self: Self) -> Vec<u8> {
        match self {
            RespFrame::SimpleString(s) => format!("+{}{}", s, CRLF).into_bytes(),
            RespFrame::Error(err) => format!("-{}{}", err, CRLF).into_bytes(),
            RespFrame::Integer(num) => {
                if num >= 0 {
                    format!(":{}{}", num, CRLF).into_bytes()
                } else {
                    format!(":-{}\r\n", num).into_bytes()
                }
            },
            RespFrame::BulkString(s) => format!("${}{}{}{}", s.len(), CRLF, s, CRLF).into_bytes(),
            RespFrame::Null => format!("_{}", CRLF).into_bytes(),
            RespFrame::NullBulkString => format!("$-1{}", CRLF).into_bytes(),
            RespFrame::NullArray => format!("*-1{}", CRLF).into_bytes(),
            RespFrame::EmptyArray => format!("*0{}", CRLF).into_bytes(),
            RespFrame::Array(items) => {
                let mut encoded = format!("*{}{}", items.len(), CRLF).into_bytes();
                for item in items {
                    encoded.extend(item.encode());
                }
                encoded
            }
            _ => todo!()
        }
    }
}