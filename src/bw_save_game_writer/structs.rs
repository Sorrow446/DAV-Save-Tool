use std::io::{BufWriter, Write};
use std::path::PathBuf;

pub struct BWSaveGameWriter<W: Write> {
    pub(crate) w: BufWriter<W>,
    pub(crate) temp_path: PathBuf,
}