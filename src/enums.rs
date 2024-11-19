use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Cmd {
    DumpBlocks,
    DumpMetadata,
}