use crate::build::library::Library;
use crate::config::Config;
use anyhow::Context;
use handlebars::Handlebars;
use itertools::Itertools;
use rust_embed::Embed;
use serde::Serialize;
use std::cmp::Reverse;
use std::fs;
use std::time::SystemTime;

pub fn render_pages(config: &Config, library: &Library) -> anyhow::Result<()> {
    let handlebars = Handlebars::new();

    let non_unique_tags = library
        .videos
        .iter()
        .flat_map(|video| &video.tags)
        .counts()
        .into_iter()
        .map(|(tag, count)| TemplateTag { tag, count })
        .sorted_by_key(|each| (Reverse(each.count), each.tag))
        .collect_vec();

    let mut videos = Vec::with_capacity(library.videos.len());

    for library_video in &library.videos {
        let tags_json = serde_json::to_string(&library_video.tags)?;

        let restriction = library_video.restriction.as_ref();
        let template_video = TemplateVideo {
            tags_json,
            tags: &library_video.tags,
            thumbnail: &library_video.thumbnail,
            video: library_video
                .restriction
                .is_none()
                .then_some(&library_video.video),
            access_rule: restriction.map(|restriction| restriction.rule.as_str()),
            access_iv: restriction.map(|restriction| restriction.iv.as_str()),
            access_ciphertext: restriction.map(|restriction| restriction.ciphertext.as_str()),
        };

        videos.push(template_video);
    }

    let template = Asset::get("index.html.hbs").context("missing template file")?;
    let template = std::str::from_utf8(&template.data)?;

    let build_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let data = TemplateData {
        build_time,
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
    build_time: u64,
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
