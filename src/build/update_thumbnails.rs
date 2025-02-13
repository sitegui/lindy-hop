use crate::config::Config;
use crate::tags_file::TagsVideo;
use crate::utils::list_files;
use anyhow::{ensure, Context};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

/// Create the necessary thumbnails, returning a mapping from video name to thumbnail name
pub fn update_thumbnails(
    config: &Config,
    videos_dir: &Path,
    videos: &[TagsVideo],
) -> anyhow::Result<BTreeMap<String, String>> {
    let mut mapping = BTreeMap::new();
    let thumbnail_dir = Path::new("data/build/thumbnails");
    fs::create_dir_all(thumbnail_dir)?;
    let existing_thumbnails = list_files(thumbnail_dir)?;
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
            .0;
        let thumbnail_hash = &video_hash[0..config.thumbnail_hex_chars_prefix];

        let thumbnail_name = format!("{}.webp", thumbnail_hash);

        mapping.insert(video.name.clone(), thumbnail_name.clone());
        if !existing_thumbnail_hashes.contains(&thumbnail_hash) {
            missing_thumbnails.push((video, thumbnail_name));
        }
    }

    log::info!("Will update {} new thumbnails", missing_thumbnails.len());
    for (video, thumbnail_name) in missing_thumbnails {
        let video_path = videos_dir.join(&video.name);
        let duration_s = measure_duration_s(&video_path).context("failed to get duration")?;

        let thumbnail_position_s = duration_s / 2.0;
        let thumbnail_path = thumbnail_dir.join(thumbnail_name);
        log::info!(
            "Will extract thumbnail at {} into {}",
            thumbnail_position_s,
            thumbnail_path.display()
        );
        create_thumbnail(config, &video_path, thumbnail_position_s, &thumbnail_path)
            .context("failed to create thumbnail")?;
    }

    Ok(mapping)
}

fn measure_duration_s(video: &Path) -> anyhow::Result<f64> {
    let output = Command::new("ffprobe")
        .args(["-of", "json", "-show_entries", "format=duration"])
        .arg(video)
        .output()
        .context("failed to execute ffprobe")?;

    ensure!(
        output.status.success(),
        "ffprobe returned a non-zero exit code"
    );

    let data: Value = serde_json::from_slice(&output.stdout)?;
    let duration = data
        .get("format")
        .and_then(|format| format.get("duration"))
        .and_then(|duration| duration.as_str())
        .and_then(|duration| duration.parse::<f64>().ok())
        .context("failed to parse duration")?;

    Ok(duration)
}

fn create_thumbnail(
    config: &Config,
    input: &Path,
    position_s: f64,
    output: &Path,
) -> anyhow::Result<()> {
    let status = Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(input)
        .arg("-ss")
        .arg(position_s.to_string())
        .arg("-frames:v")
        .arg("1")
        .arg("-vf")
        .arg(format!("scale=-1:{}", config.thumbnail_height))
        .arg(output)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("failed to execute ffmpeg")?;

    ensure!(status.success(), "ffmpeg returned a non-zero exit code");

    Ok(())
}
