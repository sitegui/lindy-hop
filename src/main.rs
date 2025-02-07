use crate::copy_new_videos::copy_new_videos;
use crate::list_mtp_mounts::list_mtp_mounts;
use clap::Parser;
use std::path::PathBuf;

mod config;
mod copy_new_videos;
mod list_mtp_mounts;

#[derive(Parser)]
enum Cli {
    /// List the MTP (media transfer protocol) mounts in this device
    ListMtpMounts,
    /// Copy new videos from the WhatsApp folder into `data/new_files`
    CopyNewVideos {
        /// The name path of the mount, extracted with `list-mtp-mounts`
        mount: PathBuf,
    },
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
    }

    Ok(())
}
