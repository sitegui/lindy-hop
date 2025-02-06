use std::fs;
use std::path::{Path, PathBuf};
use users::get_current_uid;

pub fn list_mtp_mounts() -> anyhow::Result<()> {
    let user = get_current_uid();

    log::info!("User id: {}", user);

    let mounts = list_sub_dirs(format!("/run/user/{}/gvfs", user))?;
    println!("Detected these MTP mounts:");
    for mount in mounts {
        let start = list_sub_dirs(&mount)?;
        if start.len() == 1 {
            println!("{}", start[0].display());
        }
    }

    Ok(())
}

fn list_sub_dirs(path: impl AsRef<Path>) -> anyhow::Result<Vec<PathBuf>> {
    let mut sub_dirs = Vec::new();

    for item in fs::read_dir(path)? {
        let item = item?;
        if item.file_type()?.is_dir() {
            sub_dirs.push(item.path());
        }
    }

    Ok(sub_dirs)
}
