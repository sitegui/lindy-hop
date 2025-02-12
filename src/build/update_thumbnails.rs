use crate::tags_file::TagsVideo;
use crate::utils::list_files;
use anyhow::{ensure, Context};
use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn update_thumbnails(videos_dir: &Path, videos: &[TagsVideo]) -> anyhow::Result<()> {
    let thumbnail_dir = Path::new("data/build/public/thumbnails");
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

        if !existing_thumbnail_hashes.contains(&video_hash) {
            missing_thumbnails.push((video, video_hash));
        }
    }

    log::info!("Will update {} new thumbnails", missing_thumbnails.len());
    for (video, hash) in missing_thumbnails {
        let video_path = videos_dir.join(&video.name);
        let duration_s = measure_duration_s(&video_path).context("failed to get duration")?;

        let thumbnail_position_s = duration_s / 2.0;
        let thumbnail_path = thumbnail_dir.join(format!("{}.jpg", hash));
        log::info!(
            "Will extract thumbnail at {} into {}",
            thumbnail_position_s,
            thumbnail_path.display()
        );
        create_thumbnail(&video_path, thumbnail_position_s, &thumbnail_path)
            .context("failed to create thumbnail")?;
    }

    Ok(())
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

fn create_thumbnail(input: &Path, position_s: f64, output: &Path) -> anyhow::Result<()> {
    let status = Command::new("ffmpeg")
        .arg("-i")
        .arg(input)
        .arg("-ss")
        .arg(position_s.to_string())
        .arg("-frames:v")
        .arg("1")
        .arg("-q:v")
        .arg("2")
        .arg(output)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("failed to execute ffmpeg")?;

    ensure!(status.success(), "ffmpeg returned a non-zero exit code");

    Ok(())
}
