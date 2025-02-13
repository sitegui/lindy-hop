mod encrypt;
mod ingest_tagging_in_progress;
mod library;
mod render_pages;
mod restrictions;
mod update_thumbnails;

use crate::build::ingest_tagging_in_progress::ingest_tagging_in_progress;
use crate::build::library::create_library;
use crate::build::render_pages::render_pages;
use crate::build::restrictions::Restrictions;
use crate::build::update_thumbnails::update_thumbnails;
use crate::config::Config;
use crate::tags_file::TagsFile;
use crate::utils::maybe_read_string;
use anyhow::Context;
use std::fs;
use std::path::Path;

pub fn build(config: &Config) -> anyhow::Result<()> {
    let all_tags_path = "data/all_tags.txt";
    let mut all_tags: TagsFile = maybe_read_string(all_tags_path)?
        .unwrap_or_default()
        .parse()
        .context("failed to parse data/tags.txt")?;

    log::info!("Read existing tags for {} videos", all_tags.videos.len());
    let ingest_result = ingest_tagging_in_progress(&mut all_tags);

    // In all cases, persist the new tags
    let write_result = fs::write(all_tags_path, all_tags.to_string());
    ingest_result?;
    write_result?;

    let thumbnails = update_thumbnails(config, Path::new("data/build/videos"), &all_tags.videos)?;

    let restrictions = match maybe_read_string("data/restrictions.json")? {
        None => Restrictions::default(),
        Some(data) => serde_json::from_str(&data)?,
    };
    let library = create_library(config, &all_tags, &restrictions, &thumbnails)?;

    fs::write("data/library.json", serde_json::to_string_pretty(&library)?)?;

    log::info!("Will render final page");
    render_pages(config, &library)?;

    Ok(())
}
