use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// Reads a file into memory. Returns `None` if the file does not exist
pub fn maybe_read_string(path: impl AsRef<Path>) -> anyhow::Result<Option<String>> {
    let path = path.as_ref();

    match fs::read_to_string(path) {
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(None),
        Ok(data) => Ok(Some(data)),
        Err(error) => {
            Err(anyhow::Error::from(error).context(format!("failed to read {}", path.display())))
        }
    }
}

pub fn list_files(path: impl AsRef<Path>) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for item in fs::read_dir(path)? {
        let item = item?;
        if item.file_type()?.is_file() {
            files.push(item.path());
        }
    }

    Ok(files)
}

pub fn list_dirs(path: impl AsRef<Path>) -> anyhow::Result<Vec<PathBuf>> {
    let mut dirs = Vec::new();

    for item in fs::read_dir(path)? {
        let item = item?;
        if item.file_type()?.is_dir() {
            dirs.push(item.path());
        }
    }

    Ok(dirs)
}
