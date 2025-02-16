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
    let mut handlebars = Handlebars::new();
    handlebars.register_partial("head", asset_data("partials/head.html.hbs")?)?;
    handlebars.register_partial("video", asset_data("partials/video.html.hbs")?)?;

    let data = template_data(config, library)?;
    let index_template = asset_data("index.html.hbs")?;
    let rendered = handlebars.render_template(&index_template, &data)?;
    fs::write("data/build/index.html", rendered)?;

    let video_template = asset_data("video_page.html.hbs")?;
    fs::create_dir_all("data/build/video")?;
    for video in &data.videos {
        let rendered = handlebars.render_template(
            &video_template,
            &SingleVideoTemplateData {
                public_url: &config.public_url,
                page_title: format!("Vidéo Lindy Hop - {}", video.tags.iter().format(", ")),
                build_time: data.build_time,
                access_salt: data.access_salt,
                access_iterations: data.access_iterations,
                video,
            },
        )?;
        fs::write(
            format!("data/build/video/{}.html", video.short_name),
            rendered,
        )?;
    }

    fs::write("data/build/css.css", asset_data("css.css")?)?;
    fs::write("data/build/js.mjs", asset_data("js.mjs")?)?;
    fs::write("data/build/favicon.png", asset_binary_data("favicon.png")?)?;

    Ok(())
}

fn asset_binary_data(name: &str) -> anyhow::Result<Vec<u8>> {
    let file = Asset::get(name).with_context(|| format!("missing static file {}", name))?;
    Ok(file.data.into_owned())
}

fn asset_data(name: &str) -> anyhow::Result<String> {
    let data = String::from_utf8(asset_binary_data(name)?)?;
    Ok(data)
}

fn template_data<'a>(
    config: &'a Config,
    library: &'a Library,
) -> anyhow::Result<IndexTemplateData<'a>> {
    let all_tags = library
        .videos
        .iter()
        .flat_map(|video| &video.tags)
        .counts()
        .into_iter()
        .map(|(tag, count)| TemplateTag { tag, count })
        .sorted_by_key(|each| (each.tag, Reverse(each.count)))
        .collect_vec();

    let mut videos = Vec::with_capacity(library.videos.len());

    for library_video in &library.videos {
        let tags_json = serde_json::to_string(&library_video.tags)?;

        let restriction = library_video.restriction.as_ref();
        let short_name = &library_video.video[0..config.thumbnail_hex_chars_prefix];
        let template_video = TemplateVideo {
            tags_json,
            tags: &library_video.tags,
            short_name,
            thumbnail: format!("thumbnails/{}", library_video.thumbnail),
            video: library_video
                .restriction
                .is_none()
                .then_some(&library_video.video),
            access_rule: restriction.map(|restriction| restriction.rule.as_str()),
            access_iv: restriction.map(|restriction| restriction.iv.as_str()),
            access_ciphertext: restriction.map(|restriction| restriction.ciphertext.as_str()),
            share_link: format!("video/{}.html", short_name),
        };

        videos.push(template_video);
    }

    let build_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    Ok(IndexTemplateData {
        build_time,
        all_tags,
        access_salt: &config.file_access_salt,
        access_iterations: config.file_access_iterations,
        videos,
    })
}

#[derive(Embed)]
#[folder = "web_src"]
struct Asset;

#[derive(Debug, Serialize)]
struct IndexTemplateData<'a> {
    build_time: u64,
    access_salt: &'a str,
    access_iterations: u32,
    videos: Vec<TemplateVideo<'a>>,
    all_tags: Vec<TemplateTag<'a>>,
}

#[derive(Debug, Serialize)]
struct SingleVideoTemplateData<'a> {
    public_url: &'a str,
    page_title: String,
    build_time: u64,
    access_salt: &'a str,
    access_iterations: u32,
    video: &'a TemplateVideo<'a>,
}

#[derive(Debug, Serialize)]
struct TemplateVideo<'a> {
    tags_json: String,
    tags: &'a [String],
    short_name: &'a str,
    thumbnail: String,
    video: Option<&'a str>,
    access_rule: Option<&'a str>,
    access_iv: Option<&'a str>,
    access_ciphertext: Option<&'a str>,
    share_link: String,
}

#[derive(Debug, Serialize)]
struct TemplateTag<'a> {
    tag: &'a str,
    count: usize,
}
