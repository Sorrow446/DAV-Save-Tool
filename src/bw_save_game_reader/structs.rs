use std::io::{BufReader, Read, Seek};

pub struct BWSaveGameReader<R: Read + Seek> {
    pub(crate) f: BufReader<R>,
    pub(crate) buffer: Vec<u8>,
    pub(crate) header: Header,
    pub(crate) block_one_data: Vec<u8>,
    pub(crate) block_two_data: Vec<u8>,
}

#[derive(Default)]
pub struct Header {
    #[allow(dead_code)]
    pub(crate) version: u32,
    pub(crate) _unk_001: [u8; 4],
    pub(crate) _unk_002: [u8; 8],
    pub(crate) block_one_comp_size: u64,
    pub(crate) block_one_decomp_size: u64,
    pub(crate) block_two_comp_size: u64,
    pub(crate) block_two_decomp_size: u64,
}