use std::error::Error;
use std::io;
use std::io::{BufReader, Read, Seek};
use flate2::bufread::GzDecoder;
use crate::csav::structs::{CSAVReader, Header};

// <!--DASC
const MAGIC: &[u8; 8] = b"<!--DASC";

impl Default for Header {
    fn default() -> Self {
        let unk_001: [u8; 4] = [0; 4];
        let unk_002: [u8; 8] = [0; 8];
        Header {
            version: 0,
            _unk_001: unk_001,
            _unk_002: unk_002,
            block_one_comp_size: 0,
            block_one_decomp_size: 0,
            block_two_comp_size: 0,
            block_two_decomp_size: 0,
        }
    }
}

// Vec buf needed because of dyn sizes.
impl<R: Read + Seek> CSAVReader<R> {
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

    fn read_bytes(&mut self, count: usize) -> io::Result<&[u8]> {
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

    fn read_u32_le(&mut self) -> Result<u32, Box<dyn Error>> {
        let buf = self.read_bytes(4)?;
        let arr: [u8; 4] = buf.try_into()?;
        Ok(u32::from_le_bytes(arr))
    }

    fn read_u64_le(&mut self) -> Result<u64, Box<dyn Error>> {
        let buf = self.read_bytes(8)?;
        let arr: [u8; 8] = buf.try_into()?;
        Ok(u64::from_le_bytes(arr))
    }

    pub(crate) fn read_block_one(&mut self) -> io::Result<()> {
        let comp_size = self.header.block_one_comp_size as usize;
        let decomp_size = self.header.block_one_decomp_size as usize;

        let comp_data = self.read_bytes(comp_size)?;

        let mut decomp_data = vec![0; decomp_size];
        let mut decoder = GzDecoder::new(&comp_data[..]);
        decoder.read_exact(&mut decomp_data)?;

        self.block_one_data = decomp_data;

        Ok(())
    }

    pub(crate) fn read_block_two(&mut self) -> io::Result<()> {
        let comp_size = self.header.block_two_comp_size as usize;
        let decomp_size = self.header.block_two_decomp_size as usize;

        let comp_data = self.read_bytes(comp_size)?;

        let mut decomp_data = vec![0; decomp_size];
        let mut decoder = GzDecoder::new(&comp_data[..]);
        decoder.read_exact(&mut decomp_data)?;

        self.block_two_data = decomp_data;

        Ok(())
    }

}