use crate::tags_file::{TagsFile, TagsVideo};
use crate::utils::{list_dirs, maybe_read_string};
use anyhow::Context;
use sha2::{Digest, Sha256};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn build() -> anyhow::Result<()> {
    ensure_build_dirs()?;

    let all_tags_path = "data/build/tags.txt";
    let mut all_tags: TagsFile = maybe_read_string(all_tags_path)?
        .unwrap_or_default()
        .parse()
        .context("failed to parse data/build/tags.txt")?;

    log::info!("Read existing tags for {} videos", all_tags.videos.len());
    let ingest_result = ingest(&mut all_tags);

    // In all cases, persist the new tags
    let write_result = fs::write(all_tags_path, all_tags.to_string());
    ingest_result?;
    write_result?;

    Ok(())
}

fn ensure_build_dirs() -> anyhow::Result<()> {
    fs::create_dir_all("data/build/public/videos")?;
    fs::create_dir_all("data/build/public/thumbnails")?;
    fs::create_dir_all("data/build/public/overviews")?;
    Ok(())
}

fn ingest(all_tags: &mut TagsFile) -> anyhow::Result<()> {
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

    tags.videos = pending_videos;
    fs::write(tags_path, tags.to_string())?;

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

    let hash = hash_file(&source)?;
    let new_name = format!("{}.{}", hash, extension);
    let destination = format!("data/build/public/videos/{}", new_name);

    log::info!("Move {} to {}", source.display(), destination);
    fs::rename(&source, &destination)?;

    all_tags.videos.push(TagsVideo {
        name: new_name,
        tags: video.tags,
    });

    Ok(())
}

fn hash_file(path: &Path) -> anyhow::Result<String> {
    let mut hasher = Sha256::new();
    let mut file = File::open(path)?;
    let mut buf = [0; 4096];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }

        hasher.update(&buf[0..n]);
    }
    let hash = base16ct::lower::encode_string(&hasher.finalize());
    Ok(hash)
}

/*
- tagging_in_progress
    - part-0
        - tags.txt
        - > video
- build
    - tags.txt
    - public
        - videos
            - > sha256 of content
        - thumbnails
            - > by sha256 of video
        - overviews
            - > by sha256 of video
        - data.json
        - index.html
*/
