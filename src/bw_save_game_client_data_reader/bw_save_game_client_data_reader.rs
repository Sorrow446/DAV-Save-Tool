use std::error::Error;
use std::io;
use std::io::{Cursor, Read, Seek, SeekFrom};
use crate::bw_save_game_client_data_reader::structs::BWSaveGameClientDataReader;

impl BWSaveGameClientDataReader {
    pub(crate) fn new(data: Vec<u8>) -> Self {
        Self {
            c: Cursor::new(data),
            buffer: vec![0; 1024 * 1024],
            char_data: Vec::new(),
            char_data_size: 0,
            char_data_offset: 0,
            server_offset: 0,
            server_offset_is_two_bytes: false,

        }
    }

    pub(crate) fn seek_from_start(&mut self, n: u64) -> io::Result<u64> {
        self.c.seek(SeekFrom::Start(n))
    }

    fn seek_from_current(&mut self, n: i64) -> io::Result<u64> {
        self.c.seek(SeekFrom::Current(n))
    }
    pub(crate) fn read_u8(&mut self) -> io::Result<u8> {
        let mut buf = [0; 1];
        self.c.read_exact(&mut buf)?;
        Ok(buf[0])
    }
    pub(crate) fn read_bytes(&mut self, count: usize) -> io::Result<&[u8]> {
        self.c.read_exact(&mut self.buffer[..count])?;
        Ok(&self.buffer[..count])
    }

    fn unshift_size(bytes: &[u8]) -> u32 {
        let mut size = 0u32;
        let mut shift = 0;

        for &byte in bytes.iter() {
            size |= ((byte & 0x7F) as u32) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
        }

        size
    }

     fn seek_to_char_data(&mut self) -> Result<(), Box<dyn Error>>{

        for _ in 0..2 {
            let marker = self.read_u8()?;
            if marker != 0x82 {
                return Err("bad chunk start marker byte".into());
            }


            let shifted_size = self.read_bytes(2)?;

            let unshifted_size = Self::unshift_size(shifted_size);


            self.seek_from_current(unshifted_size as i64)?;
        }

        Ok(())
    }

    pub(crate) fn read_char_data(&mut self) -> Result<(), Box<dyn Error>> {
        self.seek_from_start(12)?;


        let arr = self.read_bytes(4)?;

        let is_two_bytes = arr[3] == 0x63;

        let server_offset_len = if is_two_bytes { 2 } else { 3 };

        // 0x02 server
        let unshifted_server_offset = Self::unshift_size(&arr[..server_offset_len]);

        self.seek_from_current(14)?;

        if !is_two_bytes {
            self.seek_from_current(2)?;
        }

        self.seek_to_char_data()?;
        let char_data_offset = self.c.stream_position()?;

        let marker = self.read_u8()?;
        if marker != 0x82 {
            return Err("bad chunk start marker byte".into());
        }

        let shifted_size = self.read_bytes(3)?;

        let unshifted_size = Self::unshift_size(shifted_size);
        self.seek_from_current(-4)?;

        let unshifted_size_usize = (unshifted_size+4) as usize;
        let char_data = self.read_bytes(unshifted_size_usize)?;

        self.char_data = char_data.to_vec();
        self.char_data_offset = char_data_offset;
        self.char_data_size = unshifted_size;
        self.server_offset = unshifted_server_offset;
        self.server_offset_is_two_bytes = is_two_bytes;

        Ok(())
    }


}