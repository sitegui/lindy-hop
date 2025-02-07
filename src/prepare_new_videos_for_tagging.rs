use crate::tags_file::{TagsFile, TagsVideo};
use crate::utils::list_files;
use anyhow::Context;
use std::fs;
use std::path::PathBuf;

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

        let mut tags_file = TagsFile::default();

        log::info!("Moving {} videos to {}", chunk.len(), tagging_dir.display());
        fs::create_dir_all(&tagging_dir)?;
        for file in chunk {
            let file_name = file
                .file_name()
                .context("missing file_name")?
                .to_str()
                .context("invalid file_name")?;

            fs::rename(file, tagging_dir.join(file_name))?;

            tags_file.videos.push(TagsVideo::new(file_name.to_string()));
        }

        fs::write(tagging_dir.join("tags.txt"), tags_file.to_string())?;
    }

    Ok(())
}
