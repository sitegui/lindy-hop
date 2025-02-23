use crate::utils::list_files;
use anyhow::{ensure, Context};
use serde::Deserialize;
use std::path::Path;
use std::process::Command;

pub fn re_encode_videos(max_width: i32, max_height: i32) -> anyhow::Result<()> {
    let videos = list_files("data/videos")?;

    for video in videos {
        let info = video_information(&video)?;

        eprintln!("info = {:?}", info);
    }

    Ok(())
}

#[derive(Debug)]
struct VideoInformation {
    width: i32,
    height: i32,
    fps: f64,
    audio_frequency: f64,
    size_bytes: u64,
    duration_seconds: f64,
}

fn video_information(video: &Path) -> anyhow::Result<VideoInformation> {
    let output = Command::new("ffprobe")
        .args([
            "-of",
            "json",
            "-show_entries",
            "stream=width,height,avg_frame_rate,sample_rate,codec_type:format=duration,size",
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
        sample_rate: Option<String>,
    }

    #[derive(Debug, Deserialize)]
    struct OutputJsonFormat {
        duration: String,
        size: String,
    }

    let data: OutputJson = serde_json::from_str(command_output)?;
    ensure!(data.streams.len() == 2);
    ensure!(data.streams[0].codec_type == "video");
    ensure!(data.streams[1].codec_type == "audio");

    let width = data.streams[0].width.context("missing width")?;
    let height = data.streams[0].height.context("missing height")?;
    let fps_fraction = data.streams[0]
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

    let audio_frequency = data.streams[1]
        .sample_rate
        .as_ref()
        .context("missing sample_rate")?
        .parse()
        .context("invalid sample_rate")?;

    let size_bytes = data.format.size.parse().context("size")?;

    let duration_seconds = data.format.duration.parse().context("invalid duration")?;

    Ok(VideoInformation {
        width,
        height,
        fps,
        audio_frequency,
        size_bytes,
        duration_seconds,
    })
}
