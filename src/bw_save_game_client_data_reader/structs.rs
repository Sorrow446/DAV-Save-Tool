use std::io::Cursor;

pub struct BWSaveGameClientDataReader {
    pub(crate) c: Cursor<Vec<u8>>,
    pub(crate) buffer: Vec<u8>,
    pub(crate) char_data: Vec<u8>,
    pub(crate) char_data_offset: u64,
    pub(crate) char_data_size: u32,
    pub(crate) server_offset: u32,
    pub(crate) server_offset_is_two_bytes: bool,
}