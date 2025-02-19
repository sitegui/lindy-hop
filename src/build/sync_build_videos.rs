use crate::utils::list_files;
use std::fs;
use std::path::Path;

pub fn sync_build_videos(source_dir: &Path, destination_dir: &Path) -> anyhow::Result<()> {
    fs::create_dir_all(destination_dir)?;

    let videos = list_files(source_dir)?;

    for source in videos {
        let destination = destination_dir.join(source.strip_prefix(source_dir)?);
        if !fs::exists(&destination)? {
            fs::copy(source, destination)?;
        }
    }

    Ok(())
}
