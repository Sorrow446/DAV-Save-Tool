use clap::ValueEnum;

#[derive(ValueEnum, Clone, Debug)]
pub enum Cmd {
    Db,
    DumpBlocks,
    Dm,
    DumpMetadata,
    Ia,
    InjectAppearance,
}