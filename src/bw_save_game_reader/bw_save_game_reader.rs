use std::error::Error;
use std::io;
use std::io::{BufReader, Read, Seek};
use flate2::bufread::GzDecoder;
use crate::bw_save_game_reader::structs::*;

// <!--DASC
const MAGIC: &[u8; 8] = b"<!--DASC";

// Vec buf needed because of dyn sizes.
impl<R: Read + Seek> BWSaveGameReader<R> {
    pub(crate) fn new(f: R) -> Self {
        Self {
            f: BufReader::new(f),
            // 1 MB just in case.
            buffer: vec![0; 1024 * 1024],
            header: Header::default(),
            block_one_data: Vec::new(),
            block_two_data: Vec::new(),
        }
    }

    pub(crate) fn read_bytes(&mut self, count: usize) -> io::Result<&[u8]> {
        self.f.read_exact(&mut self.buffer[..count])?;
        Ok(&self.buffer[..count])
    }

    pub(crate) fn read_header(&mut self) -> Result<(), Box<dyn Error>> {
        let buf = self.read_bytes(8)?;
        if buf != MAGIC {
            return Err("bad header magic".into());
        }

        let version = self.read_u32_le()?;

        let unk_001 = self.read_bytes(4)?.try_into()?;

        let block_two_decomp_size = self.read_u64_le()?;
        let block_two_comp_size = self.read_u64_le()?;

        let block_one_decomp_size = self.read_u64_le()?;
        let block_one_comp_size = self.read_u64_le()?;

        let unk_002 = self.read_bytes(8)?.try_into()?;

        let header = Header {
            version,
            _unk_001: unk_001,
            block_two_decomp_size,
            _unk_002: unk_002,
            block_two_comp_size,
            block_one_decomp_size,
            block_one_comp_size,
        };

        self.header = header;
        Ok(())
    }

    pub(crate) fn read_u32_le(&mut self) -> Result<u32, Box<dyn Error>> {
        let buf = self.read_bytes(4)?;
        let arr: [u8; 4] = buf.try_into()?;
        Ok(u32::from_le_bytes(arr))
    }

    pub(crate) fn read_u64_le(&mut self) -> Result<u64, Box<dyn Error>> {
        let buf = self.read_bytes(8)?;
        let arr: [u8; 8] = buf.try_into()?;

        Ok(u64::from_le_bytes(arr))
    }

    pub(crate) fn read_block_one_data(&mut self) -> io::Result<()> {
        let comp_size = self.header.block_one_comp_size as usize;
        let decomp_size = self.header.block_one_decomp_size as usize;

        let comp_data = self.read_bytes(comp_size)?;

        let mut decomp_data = vec![0; decomp_size];
        let mut decoder = GzDecoder::new(&comp_data[..]);
        decoder.read_exact(&mut decomp_data)?;

        self.block_one_data = decomp_data;

        Ok(())
    }

    pub(crate) fn read_block_two_data(&mut self) -> io::Result<()> {
        let comp_size = self.header.block_two_comp_size as usize;
        let decomp_size = self.header.block_two_decomp_size as usize;

        let comp_data = self.read_bytes(comp_size)?;

        let mut decomp_data = vec![0; decomp_size];
        let mut decoder = GzDecoder::new(&comp_data[..]);
        decoder.read_exact(&mut decomp_data)?;

        self.block_two_data = decomp_data;

        Ok(())
    }

    // pub(crate) fn read_name_and_value_u16(&mut self) -> Result<NameValueU16, Box<dyn Error>> {
    //     let (name_prefix, name) = self.read_name_string()?;
    //     let value = self.read_u16_le()?;
    //
    //     let obj = NameValueU16 {
    //         name,
    //         name_prefix,
    //         value,
    //     };
    //
    //     Ok(obj)
    // }
    //
    // pub(crate) fn read_name_and_value_u32(&mut self) -> Result<NameValueU32, Box<dyn Error>> {
    //     let (name_prefix, name) = self.read_name_string()?;
    //     let value = self.read_u32_le()?;
    //
    //     let obj = NameValueU32 {
    //         name,
    //         name_prefix,
    //         value,
    //     };
    //
    //     Ok(obj)
    // }
    //
    // pub(crate) fn read_name_and_value_i32(&mut self) -> Result<NameValueI32, Box<dyn Error>> {
    //     let (name_prefix, name) = self.read_name_string()?;
    //     let value = self.read_i32_le()?;
    //
    //     let obj = NameValueI32 {
    //         name,
    //         name_prefix,
    //         value,
    //     };
    //
    //     Ok(obj)
    // }
    //
    // pub(crate) fn read_name_and_value_i64(&mut self) -> Result<NameValueI64, Box<dyn Error>> {
    //     let (name_prefix, name) = self.read_name_string()?;
    //     let value = self.read_i64_le()?;
    //
    //     let obj = NameValueI64 {
    //         name,
    //         name_prefix,
    //         value,
    //     };
    //
    //     Ok(obj)
    // }
    //
    // pub(crate) fn read_name_and_value_sol(&mut self, n: usize) -> Result<NameValueString, Box<dyn Error>> {
    //     let (name_prefix, name) = self.read_name_string()?;
    //     let value = self.read_string_of_len(n)?;
    //
    //     let obj = NameValueString {
    //         name,
    //         name_prefix,
    //         value,
    //         value_prefix: None,
    //     };
    //
    //     Ok(obj)
    // }
    //
    // pub(crate) fn read_name_and_value_guid(&mut self) -> Result<NameValueGUID, Box<dyn Error>> {
    //     let (name_prefix, name) = self.read_name_string()?;
    //     let value = self.read_guid()?;
    //
    //     let obj = NameValueGUID {
    //         name,
    //         name_prefix,
    //         value,
    //     };
    //
    //     Ok(obj)
    // }
    //
    // pub(crate) fn read_name_and_value_date_time(&mut self) -> Result<NameValueDateTime, Box<dyn Error>> {
    //     let (name_prefix, name) = self.read_name_string()?;
    //     let value = self.read_date_time()?;
    //
    //     let obj = NameValueDateTime {
    //         name,
    //         name_prefix,
    //         value,
    //     };
    //
    //     Ok(obj)
    // }
    //
    // pub(crate) fn read_name_and_value_string(&mut self) -> Result<NameValueString, Box<dyn Error>> {
    //     let (name_prefix, name) = self.read_name_string()?;
    //
    //     // Skip length of string.
    //     self.seek_from_current(1)?;
    //     let value = self.read_null_terminated_string()?;
    //
    //     let obj = NameValueString {
    //         name,
    //         name_prefix,
    //         value,
    //         value_prefix: None,
    //     };
    //
    //     Ok(obj)
    // }
    //
    // pub(crate) fn read_name_and_value_bool(&mut self) -> Result<NameValueBool, Box<dyn Error>> {
    //     let (name_prefix, name) = self.read_name_string()?;
    //
    //     let value = self.read_u8()?;
    //     let value_bool = match value {
    //         0x0 => false,
    //         0x1 => true,
    //         _ => return Err(format!("Invalid bool value: {:#X}", value).into()),
    //     };
    //
    //     let obj = NameValueBool {
    //         name,
    //         name_prefix,
    //         value: value_bool,
    //     };
    //
    //     Ok(obj)
    // }

}