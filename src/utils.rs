use std::env;
use std::error::Error;
use std::path::PathBuf;

pub fn get_exe_path() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = env::current_exe()?;
    let parent_dir = exe_path.parent()
        .ok_or("failed to get path of executable")?;
    let exe_path_buf = PathBuf::from(parent_dir);
    Ok(exe_path_buf)
}