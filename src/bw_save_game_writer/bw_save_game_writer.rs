use std::error::Error;
use std::fs::File;
use std::{env, fs, io};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use flate2::Compression;
use flate2::write::GzEncoder;
use crate::bw_save_game_client_data_reader::structs::BWSaveGameClientDataReader;
use crate::bw_save_game_reader::structs::BWSaveGameReader;
use crate::bw_save_game_reader::bw_save_game_reader::MAGIC;
use crate::bw_save_game_writer::structs::BWSaveGameWriter;


impl BWSaveGameWriter<File> {
    pub(crate) fn new() -> io::Result<Self> {
        let temp_path = env::temp_dir().join("davst_temp.bin");
        let f = File::create(&temp_path)?;
        let w = BufWriter::new(f);

        Ok(Self { w, temp_path })
    }

    fn shift_chunk_size(size: u32) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut value = size;

        loop {
            let byte = (value & 0x7F) as u8;
            value >>= 7;

            if value > 0 {
                bytes.push(byte | 0x80);
            } else {
                bytes.push(byte);
                break;
            }
        }

        bytes
    }

    fn compress(to_comp: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(to_comp)?;

        let compressed_data = encoder.finish()?;
        Ok(compressed_data)
    }

    pub(crate) fn rebuild(&mut self, dest_save_path: &PathBuf, src_block_two_data: Vec<u8>) ->  Result<(), Box<dyn Error>> {
        let mut src_data_r = BWSaveGameClientDataReader::new(src_block_two_data);
        src_data_r.read_char_data()?;

        {
            let dest_f = File::open(&dest_save_path)?;
            let mut dest_r = BWSaveGameReader::new(dest_f);

            dest_r.read_header()?;
            dest_r.read_block_one_data()?;
            dest_r.read_block_two_data()?;

            let dest_block_data = dest_r.block_two_data;
            let mut dest_data_r = BWSaveGameClientDataReader::new(dest_block_data.clone());
            dest_data_r.read_char_data()?;

            let head = dest_block_data[..dest_data_r.char_data_offset as usize].to_vec();

            let end_of_char_data_offset = (dest_data_r.char_data_offset + dest_data_r.char_data_size as u64) + 4;

            let mut tail = dest_block_data[end_of_char_data_offset as usize..].to_vec();

            let tail_byte_needed = tail[1] == 0x02;
            if  tail_byte_needed {
                tail.insert(0, 0x0);
            }

            let mut final_data: Vec<u8> = Vec::new();

            final_data.extend_from_slice(&head);
            final_data.extend_from_slice(&src_data_r.char_data);
            final_data.extend_from_slice(&tail);

            let mut server_offset = dest_data_r.server_offset as usize - dest_data_r.char_data_size as usize + src_data_r.char_data.len() - 4;
            if tail_byte_needed {
                server_offset += 1;
            }

            let mut final_size = final_data.len() - 4;
            if dest_data_r.server_offset_is_two_bytes {
                final_data.insert(12, 0x0);
                final_data.insert(29, 0x0);
                final_size += 2;
                server_offset += 1;
            }

            let server_data_offset_shifted = Self::shift_chunk_size(server_offset as u32);
            let contrib_offset = server_offset - 18;

            let contrib_shifted = Self::shift_chunk_size(contrib_offset as u32);

            let size_bytes = Self::shift_chunk_size(final_size as u32);

            final_data[1..4].copy_from_slice(&size_bytes);
            final_data[12..15].copy_from_slice(&server_data_offset_shifted);
            final_data[29..32].copy_from_slice(&contrib_shifted);


            self.w.write_all(MAGIC)?;
            self.w.write_all(&[0x02, 0x00, 0x00, 0x00])?;
            self.w.write_all(&dest_r.header._unk_001)?;


            let block_one_comp_data = Self::compress(&dest_r.block_one_data)?;
            let block_two_comp_data = Self::compress(&final_data)?;

            let block_two_uncomp_size_bytes: &[u8; 8] = &final_data.len().to_le_bytes();
            self.w.write_all(block_two_uncomp_size_bytes)?;

            let block_two_comp_size_bytes: &[u8; 8] = &block_two_comp_data.len().to_le_bytes();
            self.w.write_all(block_two_comp_size_bytes)?;

            let block_one_uncomp_size_bytes: &[u8; 8] = &dest_r.header.block_one_decomp_size.to_le_bytes();
            self.w.write_all(block_one_uncomp_size_bytes)?;

            let block_one_comp_size_bytes: &[u8; 8] = &block_one_comp_data.len().to_le_bytes();
            self.w.write_all(block_one_comp_size_bytes)?;

            self.w.write_all(&dest_r.header._unk_002)?;

            self.w.write_all(&block_one_comp_data)?;
            self.w.write_all(&block_two_comp_data)?;
        }

        fs::remove_file(dest_save_path)?;
        fs::copy(&self.temp_path, &dest_save_path)?;

        Ok(())
    }

}