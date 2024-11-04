use std::fs::{self, File};
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

use crate::structs::{Args, Cmd, Config};
use clap::Parser;
use crate::csav::structs::CSAVReader;

mod structs;
mod utils;
mod csav;

fn parse_config() -> Result<Config, Box<dyn Error>> {
    let args = Args::parse();

    let exe_path = utils::get_exe_path()?;
    let mut out_path = args.out_path.unwrap_or(exe_path.clone());

    if !out_path.is_absolute() {
        out_path = exe_path.join(out_path);
    }

    let config = Config {
        in_path: args.in_path,
        out_path,
        command: args.command,
    };

    Ok(config)
}

fn write_block_to_file(out_path: &PathBuf, data: &[u8]) -> Result<(), Box<dyn Error>>  {
    let mut f = File::create(out_path)?;
    f.write_all(data)?;
    Ok(())
}

fn make_block_out_path(in_path: &PathBuf, out_path: &PathBuf, fname: &str) -> PathBuf {
    let mut modified_in_path = in_path.clone();
    modified_in_path.set_extension(fname);
    let save_filename_no_ext = modified_in_path.file_name().unwrap();
    out_path.join(save_filename_no_ext)
}

fn dump_blocks(config: &Config) -> Result<(), Box<dyn Error>>  {
    let f = File::open(&config.in_path)?;
    let mut csav = CSAVReader::new(f);

    println!("Reading header...");
    csav.read_header()?;

    println!("Save version: {}", csav.header.version);

    println!("Block one compressed size: {} bytes", csav.header.block_one_comp_size);
    println!("Block one decompressed size: {} bytes", csav.header.block_one_decomp_size);

    println!("Block two compressed size: {} bytes", csav.header.block_two_comp_size);
    println!("Block two decompressed size:  {} bytes", csav.header.block_two_decomp_size);

    println!("Reading blocks...");
    csav.read_block_one()?;
    csav.read_block_two()?;

    println!("Writing blocks locally...");
    let block_one_out_path = make_block_out_path(&config.in_path, &config.out_path, "block_one.bin");
    write_block_to_file(&block_one_out_path, &csav.block_one_data)?;
    let block_two_out_path = make_block_out_path(&config.in_path, &config.out_path, "block_two.bin");
    write_block_to_file(&block_two_out_path, &csav.block_two_data)?;

    println!("-> {}", block_one_out_path.to_string_lossy());
    println!("-> {}", block_two_out_path.to_string_lossy());

    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_config()
        .expect("failed to parse args");
    fs::create_dir_all(&config.out_path)?;

    let res = match config.command {
        Cmd::DumpBlocks => dump_blocks(&config),
    };

    if let Err(e) = res {
        println!("Command failed.\n{:?}", e);
    }

    Ok(())
}
