use crate::build::build;
use crate::config::Config;
use crate::copy_new_videos::copy_new_videos;
use crate::list_mtp_mounts::list_mtp_mounts;
use crate::prepare_new_videos_for_tagging::prepare_new_videos_for_tagging;
use crate::re_encode_videos::re_encode_videos;
use clap::Parser;
use std::path::PathBuf;

mod build;
mod config;
mod copy_new_videos;
mod hash_file;
mod list_mtp_mounts;
mod prepare_new_videos_for_tagging;
mod re_encode_videos;
mod tags_file;
mod utils;

#[derive(Parser)]
enum Cli {
    /// List the MTP (media transfer protocol) mounts in this device
    ListMtpMounts,
    /// Copy new videos from the WhatsApp folder into `data/new_files`.
    CopyNewVideos {
        /// The name path of the mount, extracted with `list-mtp-mounts`
        mount: PathBuf,
    },
    /// Prepare the videos in `data/new_lindy_files` to be manually tagged
    PrepareNewVideosForTagging {
        #[clap(long, default_value_t = 10)]
        part_size: usize,
    },
    /// Ingest all new videos and tags and produce the final artifacts
    Build,
    /// Re-encode large videos to reduce storage
    ReEncodeVideos {
        /// Maximum number of pixels on the smallest dimension.
        #[clap(long, default_value_t = 1080)]
        max_lines: i32,
        /// Maximum frames per second. Above this, the `target_fps` will be used
        #[clap(long, default_value_t = 31)]
        max_fps: i32,
        /// Maximum MiB/s
        #[clap(long, default_value_t = 0.5)]
        max_mib_s: f64,
        /// Maximum MiB/s
        #[clap(long, default_value_t = 30)]
        target_fps: i32,
        /// The x264 quality to use
        #[clap(long, default_value_t = 26)]
        target_crf: i32,
    },
}

fn main() -> anyhow::Result<()> {
    let _ = dotenvy::from_path(".env");
    dotenvy::from_path("default.env")?;
    env_logger::init();

    let config = Config::from_env()?;

    let cli = Cli::parse();

    match cli {
        Cli::ListMtpMounts => list_mtp_mounts(),
        Cli::CopyNewVideos { mount } => copy_new_videos(mount),
        Cli::PrepareNewVideosForTagging { part_size } => prepare_new_videos_for_tagging(part_size),
        Cli::Build => build(&config),
        Cli::ReEncodeVideos {
            max_lines,
            max_fps,
            max_mib_s,
            target_fps,
            target_crf,
        } => re_encode_videos(max_lines, max_fps, max_mib_s, target_fps, target_crf),
    }
}
