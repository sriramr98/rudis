pub enum RespFrame {
    SimpleString(String),
    Error(String),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<RespFrame>),
    Null,
}

impl RespFrame {
    pub fn encode(self: &Self) -> Vec<u8> {
        match self {
            RespFrame::SimpleString(s) => format!("+{}\r\n", s).into_bytes(),
            RespFrame::Error(_) => todo!(),
            RespFrame::Integer(_) => todo!(),
            RespFrame::BulkString(_items) => todo!(),
            RespFrame::Array(resp_frames) => todo!(),
            RespFrame::Null => todo!(),
        }
    }
}