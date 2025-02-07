use crate::build::build;
use crate::copy_new_videos::copy_new_videos;
use crate::list_mtp_mounts::list_mtp_mounts;
use crate::prepare_new_videos_for_tagging::prepare_new_videos_for_tagging;
use clap::Parser;
use std::path::PathBuf;

mod build;
mod config;
mod copy_new_videos;
mod list_mtp_mounts;
mod prepare_new_videos_for_tagging;
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
}

fn main() -> anyhow::Result<()> {
    dotenvy::from_path(".env")?;
    dotenvy::from_path("default.env")?;
    env_logger::init();

    let cli = Cli::parse();

    match cli {
        Cli::ListMtpMounts => {
            list_mtp_mounts()?;
        }
        Cli::CopyNewVideos { mount } => {
            copy_new_videos(mount)?;
        }
        Cli::PrepareNewVideosForTagging { part_size } => {
            prepare_new_videos_for_tagging(part_size)?;
        }
        Cli::Build => {
            build()?;
        }
    }

    Ok(())
}
