use crate::build::library::{Library, LibraryFile};
use crate::config::Config;
use anyhow::Context;
use handlebars::Handlebars;
use itertools::Itertools;
use rust_embed::Embed;
use serde::Serialize;
use std::cmp::Reverse;
use std::fs;

pub fn render_pages(config: &Config, library: &Library) -> anyhow::Result<()> {
    let handlebars = Handlebars::new();

    let non_unique_tags = library
        .videos
        .iter()
        .flat_map(|video| &video.tags)
        .counts()
        .into_iter()
        .filter(|&(_, count)| count > 1)
        .map(|(tag, count)| TemplateTag { tag, count })
        .sorted_by_key(|each| (Reverse(each.count), each.tag))
        .collect_vec();

    let mut videos = Vec::with_capacity(library.videos.len());

    for library_video in &library.videos {
        let tags_json = serde_json::to_string(&library_video.tags)?;

        let template_video = match &library_video.file {
            LibraryFile::Public { video } => TemplateVideo {
                tags_json,
                tags: &library_video.tags,
                thumbnail: &library_video.thumbnail,
                video: Some(video),
                access_rule: None,
                access_iv: None,
                access_ciphertext: None,
            },
            LibraryFile::Private { access } => TemplateVideo {
                tags_json,
                tags: &library_video.tags,
                thumbnail: &library_video.thumbnail,
                video: None,
                access_rule: Some(&access.rule),
                access_iv: Some(&access.iv),
                access_ciphertext: Some(&access.ciphertext),
            },
        };

        videos.push(template_video);
    }

    let template = Asset::get("index.html.hbs").context("missing template file")?;
    let template = std::str::from_utf8(&template.data)?;

    let data = TemplateData {
        non_unique_tags,
        access_salt: &config.file_access_salt,
        access_iterations: config.file_access_iterations,
        videos,
    };
    let rendered = handlebars.render_template(template, &data)?;
    fs::write("data/build/index.html", rendered)?;

    fs::write(
        "data/build/css.css",
        Asset::get("css.css").context("missing file")?.data,
    )?;
    fs::write(
        "data/build/js.mjs",
        Asset::get("js.mjs").context("missing file")?.data,
    )?;

    Ok(())
}

#[derive(Embed)]
#[folder = "web_src"]
struct Asset;

#[derive(Debug, Serialize)]
struct TemplateData<'a> {
    access_salt: &'a str,
    access_iterations: u32,
    videos: Vec<TemplateVideo<'a>>,
    non_unique_tags: Vec<TemplateTag<'a>>,
}

#[derive(Debug, Serialize)]
struct TemplateVideo<'a> {
    tags_json: String,
    tags: &'a [String],
    thumbnail: &'a str,
    video: Option<&'a str>,
    access_rule: Option<&'a str>,
    access_iv: Option<&'a str>,
    access_ciphertext: Option<&'a str>,
}

#[derive(Debug, Serialize)]
struct TemplateTag<'a> {
    tag: &'a str,
    count: usize,
}
