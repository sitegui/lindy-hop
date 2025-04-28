use crate::utils::list_files;
use anyhow::{ensure, Context};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn re_encode_videos(
    max_lines: i32,
    max_fps: i32,
    max_mib_s: f64,
    target_fps: i32,
    target_crf: i32,
) -> anyhow::Result<()> {
    let videos = list_files("data/videos")?;
    for video in videos {
        let info = video_information(&video).with_context(|| {
            format!("Failed to get information for video {}", video.display())
        })?;

        let lines = info.width.min(info.height);
        let scale = (lines > max_lines).then_some({
            if info.width > info.height {
                Scale::ConstraintHeight(max_lines)
            } else {
                Scale::ConstraintWidth(max_lines)
            }
        });

        let mib_s = info.size_bytes as f64 / 1024. / 1024. / info.duration_seconds;

        if scale.is_some() || info.fps > max_fps as f64 || mib_s > max_mib_s {
            let original =
                Path::new("original_data/videos").join(video.strip_prefix("data/videos")?);
            re_encode(target_fps, target_crf, scale, &video, &original)?;
        }
    }

    Ok(())
}

#[derive(Debug, Serialize)]
struct VideoInformation {
    width: i32,
    height: i32,
    fps: f64,
    size_bytes: u64,
    duration_seconds: f64,
}

#[derive(Debug, Copy, Clone)]
enum Scale {
    ConstraintWidth(i32),
    ConstraintHeight(i32),
}

fn video_information(video: &PathBuf) -> anyhow::Result<VideoInformation> {
    let output = Command::new("ffprobe")
        .args([
            "-of",
            "json",
            "-show_entries",
            "stream=width,height,avg_frame_rate,codec_type:format=duration,size",
        ])
        .arg(video)
        .output()
        .context("failed to execute ffprobe")?;

    ensure!(
        output.status.success(),
        "ffprobe returned a non-zero exit code"
    );

    let stdout = String::from_utf8(output.stdout).context("invalid output encoding")?;
    parse_video_information(&stdout).with_context(|| format!("Failed to parse from:\n{}", stdout))
}

fn parse_video_information(command_output: &str) -> anyhow::Result<VideoInformation> {
    #[derive(Debug, Deserialize)]
    struct OutputJson {
        streams: Vec<OutputJsonStream>,
        format: OutputJsonFormat,
    }

    #[derive(Debug, Deserialize)]
    struct OutputJsonStream {
        codec_type: String,
        width: Option<i32>,
        height: Option<i32>,
        avg_frame_rate: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    struct OutputJsonFormat {
        duration: String,
        size: String,
    }

    let data: OutputJson = serde_json::from_str(command_output)?;
    let video_stream = data.streams.into_iter().find(|s| s.codec_type == "video").context("missing video stream")?;

    let width = video_stream.width.context("missing width")?;
    let height = video_stream.height.context("missing height")?;
    let fps_fraction = video_stream
        .avg_frame_rate
        .as_ref()
        .context("missing avg_frame_rate")?
        .split_once('/')
        .context("invalid avg_frame_rate")?;
    let fps = fps_fraction
        .0
        .parse::<f64>()
        .context("invalid avg_frame_rate")?
        / fps_fraction
            .1
            .parse::<f64>()
            .context("invalid avg_frame_rate")?;

    let size_bytes = data.format.size.parse().context("size")?;

    let duration_seconds = data.format.duration.parse().context("invalid duration")?;

    Ok(VideoInformation {
        width,
        height,
        fps,
        size_bytes,
        duration_seconds,
    })
}

fn re_encode(
    target_fps: i32,
    target_crf: i32,
    scale: Option<Scale>,
    target: &Path,
    original: &Path,
) -> anyhow::Result<()> {
    if !fs::exists(original)? {
        // Backup data
        fs::create_dir_all(original.parent().context("missing parent")?)?;
        fs::copy(target, original)?;
    }

    let mut command = Command::new("ffmpeg");
    command
        .arg("-y")
        .arg("-i")
        .arg(original)
        .arg("-c:v")
        .arg("libx264")
        .arg("-c:a")
        .arg("aac")
        .arg("-r")
        .arg(target_fps.to_string())
        .arg("-crf")
        .arg(target_crf.to_string());

    match scale {
        Some(Scale::ConstraintWidth(width)) => {
            command.arg("-vf").arg(format!("scale={}:-1", width));
        }
        Some(Scale::ConstraintHeight(height)) => {
            command.arg("-vf").arg(format!("scale=-1:{}", height));
        }
        None => {}
    }

    log::info!(
        "Will convert {} into {}",
        original.display(),
        target.display()
    );
    let status = command
        .arg(target)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("failed to execute ffmpeg")?;

    ensure!(status.success(), "ffmpeg returned a non-zero exit code");

    let original_size = fs::metadata(original)?.len() as f64 / 1024. / 1024.;
    let target_size = fs::metadata(target)?.len() as f64 / 1024. / 1024.;
    log::info!("{:.1} MiB -> {:.1} MiB", original_size, target_size);

    Ok(())
}
