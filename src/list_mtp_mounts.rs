use crate::utils::list_dirs;
use users::get_current_uid;

pub fn list_mtp_mounts() -> anyhow::Result<()> {
    let user = get_current_uid();

    log::info!("User id: {}", user);

    let mounts = list_dirs(format!("/run/user/{}/gvfs", user))?;
    println!("Detected these MTP mounts:");
    for mount in mounts {
        let start = list_dirs(&mount)?;
        if start.len() == 1 {
            println!("{}", start[0].display());
        }
    }

    Ok(())
}
