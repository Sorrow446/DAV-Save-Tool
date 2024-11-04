use std::path::PathBuf;
use clap::{Parser, ValueEnum};
#[derive(Parser)]
#[command(name = "DAV save tool")]
pub struct Args {
    #[clap(short, long, help="Input path.", required=true)]
    pub in_path: PathBuf,

    #[clap(short, long, help="Output path.")]
    pub out_path: Option<PathBuf>,

    #[arg(value_enum, help="Command.")]
    pub command: Cmd,
}

pub struct Config {
    pub in_path: PathBuf,
    pub out_path: PathBuf,
    pub command: Cmd,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Cmd {
    DumpBlocks,
}