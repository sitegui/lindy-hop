mod ingest_tagging_in_progress;
mod update_thumbnails;

use crate::build::ingest_tagging_in_progress::ingest_tagging_in_progress;
use crate::build::update_thumbnails::update_thumbnails;
use crate::tags_file::TagsFile;
use crate::utils::maybe_read_string;
use anyhow::Context;
use std::fs;
use std::path::Path;

pub fn build() -> anyhow::Result<()> {
    ensure_build_dirs()?;

    let all_tags_path = "data/build/tags.txt";
    let mut all_tags: TagsFile = maybe_read_string(all_tags_path)?
        .unwrap_or_default()
        .parse()
        .context("failed to parse data/build/tags.txt")?;

    log::info!("Read existing tags for {} videos", all_tags.videos.len());
    let ingest_result = ingest_tagging_in_progress(&mut all_tags);

    // In all cases, persist the new tags
    let write_result = fs::write(all_tags_path, all_tags.to_string());
    ingest_result?;
    write_result?;

    update_thumbnails(Path::new("data/build/public/videos"), &all_tags.videos)?;

    Ok(())
}

fn ensure_build_dirs() -> anyhow::Result<()> {
    fs::create_dir_all("data/build/public/videos")?;
    fs::create_dir_all("data/build/public/thumbnails")?;
    fs::create_dir_all("data/build/public/overviews")?;
    Ok(())
}

/*
- tagging_in_progress
    - part-0
        - tags.txt
        - > video
- build
    - tags.txt
    - public
        - videos
            - > sha256 of content
        - thumbnails
            - > by sha256 of video
        - overviews
            - > by sha256 of video
        - data.json
        - index.html
*/
