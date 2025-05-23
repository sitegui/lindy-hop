use crate::hash_file;
use crate::tags_file::{TagsFile, TagsVideo};
use crate::utils::list_dirs;
use anyhow::Context;
use std::fs;
use std::path::Path;

pub fn ingest_tagging_in_progress(all_tags: &mut TagsFile) -> anyhow::Result<()> {
    fs::create_dir_all("data/videos")?;

    for part_dir in list_dirs("data/tagging_in_progress")? {
        ingest_tags(all_tags, &part_dir)?;
    }

    Ok(())
}

fn ingest_tags(all_tags: &mut TagsFile, part_dir: &Path) -> anyhow::Result<()> {
    let tags_path = part_dir.join("tags.txt");
    let mut tags: TagsFile = fs::read_to_string(&tags_path)?.parse()?;

    let mut pending_videos = Vec::new();
    for video in tags.videos {
        if !video.tags.is_empty() {
            let name = video.name.clone();
            ingest_video(all_tags, part_dir, video).with_context(|| {
                format!("failed to ingest {} from {}", name, part_dir.display())
            })?;
        } else {
            pending_videos.push(video);
        }
    }

    if pending_videos.is_empty() {
        log::info!("finished {}", part_dir.display());
        fs::remove_file(tags_path)?;
        fs::remove_dir(part_dir)?;
    } else {
        tags.videos = pending_videos;
        fs::write(tags_path, tags.to_string())?;
    }

    Ok(())
}

fn ingest_video(all_tags: &mut TagsFile, part_dir: &Path, video: TagsVideo) -> anyhow::Result<()> {
    let source = part_dir.join(&video.name);
    let extension = video.name.rsplit_once('.').context("missing extension")?.1;
    log::info!(
        "Ingest video {} with {} tags",
        source.display(),
        video.tags.len()
    );

    let hash = hash_file::hash_file(&source)?;
    let new_name = format!("{}.{}", hash, extension);
    let destination = format!("data/videos/{}", new_name);

    if !fs::exists(&destination)? {
        log::info!("Move {} to {}", source.display(), destination);
        fs::rename(&source, &destination)?;

        all_tags.videos.push(TagsVideo {
            name: new_name,
            tags: video.tags,
        });
    } else {
        log::warn!("File {} already exists: merging their tags", destination);
        fs::remove_file(&destination)?;

        let ingested_video = all_tags
            .videos
            .iter_mut()
            .find(|video| video.name == new_name)
            .context("could not find previous tags")?;

        for tag in video.tags {
            if !ingested_video.tags.contains(&tag) {
                ingested_video.tags.push(tag);
            }
        }
    }

    Ok(())
}
