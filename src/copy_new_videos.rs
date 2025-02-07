use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

pub fn copy_new_videos(mount: PathBuf) -> anyhow::Result<()> {
    let mut files = Vec::new();

    accumulate_files(
        &mut files,
        &mount,
        &mount.join("Android/media/com.whatsapp/WhatsApp/Media/WhatsApp Video"),
    )
    .context("failed to list device files")?;

    log::info!("Detected {} files", files.len());

    let mut copied_files = match fs::read("data/copied_files.json") {
        Err(err) if err.kind() == ErrorKind::NotFound => CopiedFiles::default(),
        Ok(data) => serde_json::from_slice(&data)?,
        Err(error) => {
            return Err(
                anyhow::Error::from(error).context("failed to read parse previously copied files")
            )
        }
    };

    log::info!("{} previously copied files", copied_files.files.len());

    let mut to_copy = Vec::new();
    for file in files {
        if !copied_files.files.contains(&file) {
            to_copy.push(file);
        }
    }

    if to_copy.is_empty() {
        log::info!("No new files to copy");
        return Ok(());
    }
    log::info!("Will copy {} new files", to_copy.len());

    let new_files_dir = Path::new("data/new_files");
    fs::create_dir_all(new_files_dir).context("failed to create new_files folder")?;

    let mut successes = 0;
    for file in to_copy {
        let mut do_copy = || -> anyhow::Result<()> {
            let source = mount.join(&file.relative_path);
            let destination = detect_destination(&file.relative_path, new_files_dir)?;
            log::info!("Copying {} to {}", source.display(), destination.display());
            fs::copy(source, destination)?;
            copied_files.files.insert(file.clone());
            successes += 1;
            Ok(())
        };

        if let Err(error) =
            do_copy().with_context(|| format!("failed to copy {}", file.relative_path.display()))
        {
            log::warn!("{}", error);
        }
    }

    fs::write(
        "data/copied_files.json",
        serde_json::to_string(&copied_files)?,
    )
    .context("failed to persist new copied_files information")?;
    log::info!("{} files copied successfully", successes);

    fs::create_dir_all("data/new_lindy_files")
        .context("failed to create new_lindy_files folder")?;
    log::info!("You can now triage these videos and move to data/new_lindy_files the ones that you want to consider");

    Ok(())
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct CopiedFiles {
    files: BTreeSet<FileInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
struct FileInfo {
    relative_path: PathBuf,
    len: u64,
}

fn accumulate_files(files: &mut Vec<FileInfo>, base_path: &Path, dir: &Path) -> anyhow::Result<()> {
    for item in fs::read_dir(dir)? {
        let item = item?;
        let path = item.path();
        let metadata = item.metadata()?;

        if metadata.is_file() {
            files.push(FileInfo {
                relative_path: path.strip_prefix(base_path)?.to_owned(),
                len: metadata.len(),
            });
        } else if metadata.is_dir() {
            accumulate_files(files, base_path, &path)?;
        }
    }

    Ok(())
}

fn detect_destination(relative_source: &Path, destination_dir: &Path) -> anyhow::Result<PathBuf> {
    let file_name = relative_source
        .file_name()
        .context("missing file_name")?
        .to_str()
        .context("non-utf8 file_name")?;
    let mut trial = 1;
    loop {
        let destination = if trial == 1 {
            destination_dir.join(file_name)
        } else {
            let (stem, extension) = file_name
                .rsplit_once('.')
                .context("missing file extension")?;
            destination_dir.join(format!("{} ({}).{}", stem, trial, extension))
        };

        if !fs::exists(&destination)? {
            return Ok(destination);
        }

        trial += 1;
    }
}
