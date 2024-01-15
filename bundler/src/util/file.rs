use std::{
    fs::{self, File},
    path::Path,
};
use anyhow::{Result, Context};

pub fn create_file_with_dirs<P: AsRef<Path>>(path: P) -> Result<File> {
    if let Some(parent_dir) = path.as_ref().parent() {
        fs::create_dir_all(parent_dir)?;
    }
    File::create(path).context("failed to create file")
}
