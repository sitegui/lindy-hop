use anyhow::Context;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

pub fn prepare_new_videos_for_tagging(part_size: usize) -> anyhow::Result<()> {
    let mut new_lindy_videos = list_files("data/new_lindy_files")?;
    log::info!("Detected {} new lindy files", new_lindy_videos.len());
    new_lindy_videos.sort();
    if new_lindy_videos.is_empty() {
        log::info!("Nothing to do");
        return Ok(());
    }

    let tagging_dirs = "data/tagging_in_progress";
    fs::create_dir_all("tagging_dirs")?;

    let mut id = 0;
    for chunk in new_lindy_videos.chunks(part_size) {
        let tagging_dir = loop {
            let tagging_dir = PathBuf::from(format!("{}/part-{}", tagging_dirs, id));
            if !tagging_dir.try_exists()? {
                break tagging_dir;
            }

            id += 1;
        };

        let mut tags_contents = String::new();

        log::info!("Moving {} videos to {}", chunk.len(), tagging_dir.display());
        fs::create_dir_all(&tagging_dir)?;
        for file in chunk {
            let file_name = file
                .file_name()
                .context("missing file_name")?
                .to_str()
                .context("invalid file_name")?;

            fs::rename(file, tagging_dir.join(file_name))?;

            writeln!(tags_contents, "[{}]", file_name)?;
            writeln!(tags_contents)?;
        }

        fs::write(tagging_dir.join("tags.txt"), tags_contents)?;
    }

    Ok(())
}

fn list_files(dir: impl AsRef<Path>) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for item in fs::read_dir(dir)? {
        let item = item?;
        if item.file_type()?.is_file() {
            files.push(item.path());
        }
    }
    Ok(files)
}
