use crate::build::encrypt::encrypt;
use crate::build::restrictions::{RestrictionRule, Restrictions};
use crate::config::Config;
use crate::tags_file::{TagsFile, TagsVideo};
use anyhow::Context;
use regex::Regex;
use serde::Serialize;
use std::cmp::Reverse;
use std::collections::BTreeMap;
use std::sync::LazyLock;

#[derive(Debug, Serialize)]
pub struct Library {
    pub videos: Vec<LibraryVideo>,
}

#[derive(Debug, Serialize)]
pub struct LibraryVideo {
    pub date: Option<Date>,
    pub tags: Vec<String>,
    pub thumbnail: String,
    pub video: String,
    pub restriction: Option<LibraryRestriction>,
}

#[derive(Debug, Serialize, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, Serialize)]
pub struct LibraryRestriction {
    pub rule: String,
    pub iv: String,
    pub ciphertext: String,
}

pub fn create_library(
    config: &Config,
    tags_file: &TagsFile,
    restrictions: &Restrictions,
    thumbnails: &BTreeMap<String, String>,
) -> anyhow::Result<Library> {
    let mut library = Library {
        videos: Vec::with_capacity(tags_file.videos.len()),
    };

    for video in &tags_file.videos {
        library
            .videos
            .push(convert_video(config, restrictions, video, thumbnails)?);
    }

    library
        .videos
        .sort_by_key(|video| (Reverse(video.date), video.video.clone()));

    Ok(library)
}

fn convert_video(
    config: &Config,
    restrictions: &Restrictions,
    video: &TagsVideo,
    thumbnails: &BTreeMap<String, String>,
) -> anyhow::Result<LibraryVideo> {
    let restriction = match restrictions.find(video) {
        None => None,
        Some(rule) => Some(create_file_access(config, &video.name, rule)?),
    };

    let mut tags = video.tags.clone();
    tags.sort();

    Ok(LibraryVideo {
        date: extract_date(&video.tags),
        tags,
        video: video.name.clone(),
        thumbnail: thumbnails
            .get(&video.name)
            .context("missing thumbnail")?
            .clone(),
        restriction,
    })
}

fn extract_date(tags: &[String]) -> Option<Date> {
    static DATE_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^(\d\d\d\d)-(\d\d)-(\d\d)$").unwrap());

    for tag in tags {
        if let Some(captures) = DATE_REGEX.captures(tag) {
            let year = captures[1].parse();
            let month = captures[2].parse();
            let day = captures[3].parse();

            if let (Ok(year), Ok(month), Ok(day)) = (year, month, day) {
                return Some(Date { year, month, day });
            }
        }
    }

    None
}

fn create_file_access(
    config: &Config,
    video_name: &str,
    rule: &RestrictionRule,
) -> anyhow::Result<LibraryRestriction> {
    let encrypted = encrypt(
        &rule.password,
        &config.file_access_salt,
        config.file_access_iterations,
        video_name,
    )?;

    Ok(LibraryRestriction {
        rule: rule.name.clone(),
        iv: encrypted.iv,
        ciphertext: encrypted.ciphertext,
    })
}
