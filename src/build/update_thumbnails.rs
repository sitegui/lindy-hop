use crate::tags_file::TagsVideo;
use crate::utils::list_files;
use anyhow::{ensure, Context};
use serde_json::Value;
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

pub fn update_thumbnails(videos: &[TagsVideo]) -> anyhow::Result<()> {
    let existing_thumbnails = list_files("data/build/public/thumbnails")?;
    let mut existing_thumbnail_hashes = BTreeSet::new();
    for thumbnail in &existing_thumbnails {
        let hash = thumbnail
            .file_stem()
            .and_then(|stem| stem.to_str())
            .context("failed to get file stem")?;

        existing_thumbnail_hashes.insert(hash);
    }

    let mut missing_thumbnails = Vec::new();
    for video in videos {
        let video_hash = video
            .name
            .rsplit_once('.')
            .context("failed to get file stem")?
            .1;

        if !existing_thumbnail_hashes.contains(&video_hash) {
            missing_thumbnails.push((video, video_hash));
        }
    }

    log::info!("Will update {} new thumbnails", missing_thumbnails.len());

    Ok(())
}

fn measure_duration_s(video: &Path) -> anyhow::Result<f64> {
    let output = Command::new("ffprobe")
        .args(&["-of", "json", "-show_entries", "format=duration"])
        .arg(video)
        .output()
        .context("failed to execute ffprobe")?;

    ensure!(
        output.status.success(),
        "ffprobe returned a non-zero exit code"
    );

    let data: Value = serde_json::from_slice(&output.stdout)?;
    data.get("format");

    Ok(())
}
