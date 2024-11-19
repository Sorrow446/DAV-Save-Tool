use std::fs::{self, File};
use std::error::Error;
use std::io::Write;
use std::path::PathBuf;

use crate::structs::{Args, Config};
use clap::Parser;
use crate::bw_save_game_metadata_reader::structs::BWSaveGameMetadataReader;
use crate::bw_save_game_reader::structs::BWSaveGameReader;
use crate::enums::Cmd;

mod structs;
mod utils;
mod bw_save_game_reader;
mod bw_save_game_metadata_reader;
mod enums;


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

fn make_out_path(in_path: &PathBuf, out_path: &PathBuf, fname: &str) -> PathBuf {
    let mut modified_in_path = in_path.clone();
    modified_in_path.set_extension(fname);
    let save_filename_no_ext = modified_in_path.file_name().unwrap();
    out_path.join(save_filename_no_ext)
}

fn dump_metadata(config: &Config, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
    let mut r = BWSaveGameMetadataReader::new(data);

    println!("Parsing metadata...");
    r.parse_metadata()?;

    let out_path = make_out_path(&config.in_path, &config.out_path, "metadata.json");

    let json_data = serde_json::to_string_pretty(&r.metadata)?;

    let mut f = File::create(&out_path)?;
    f.write_all(json_data.as_bytes())?;

    println!("-> {}", out_path.to_string_lossy());

    Ok(())
}


fn dump_blocks(config: &Config, r: BWSaveGameReader<File>) -> Result<(), Box<dyn Error>>  {

    println!("Writing blocks locally...");
    let block_one_out_path = make_out_path(&config.in_path, &config.out_path, "block_one.bin");
    write_block_to_file(&block_one_out_path, &r.block_one_data)?;
    let block_two_out_path = make_out_path(&config.in_path, &config.out_path, "block_two.bin");
    write_block_to_file(&block_two_out_path, &r.block_two_data)?;

    println!("-> {}", block_one_out_path.to_string_lossy());
    println!("-> {}", block_two_out_path.to_string_lossy());

    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_config()
        .expect("failed to parse args");
    fs::create_dir_all(&config.out_path)?;

    let f = File::open(&config.in_path)?;
    let mut r = BWSaveGameReader::new(f);

    println!("Reading header...");
    r.read_header()?;

    println!("Reading blocks...");
    r.read_block_one_data()?;
    r.read_block_two_data()?;


    let res = match config.command {
        Cmd::DumpBlocks => dump_blocks(&config, r),
        Cmd::DumpMetadata => dump_metadata(&config, r.block_one_data),
    };

    if let Err(e) = res {
        println!("Command failed.\n{:?}", e);
    }

    Ok(())
}
