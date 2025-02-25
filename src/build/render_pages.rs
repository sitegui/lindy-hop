use crate::build::library::Library;
use crate::config::Config;
use anyhow::Context;
use handlebars::Handlebars;
use itertools::Itertools;
use rust_embed::Embed;
use serde::Serialize;
use std::fs;
use std::time::SystemTime;
use unidecode::unidecode;

pub fn render_pages(config: &Config, library: &Library) -> anyhow::Result<()> {
    let handlebars = handlebars()?;

    let home_data = home_page_data(config, library)?;
    let rendered = handlebars.render("home_page", &home_data)?;
    fs::write("build/index.html", rendered)?;

    let _ = fs::remove_dir_all("build/video");
    fs::create_dir_all("build/video")?;
    for video in &home_data.videos {
        let video_data = VideoPageData {
            public_url: &config.public_url,
            page_title: format!(
                "Vidéo Lindy Hop - {}",
                video.tags.iter().map(|tag| &tag.name).format(", ")
            ),
            build_time: home_data.build_time,
            access_salt: home_data.access_salt,
            access_iterations: home_data.access_iterations,
            video,
        };
        let rendered = handlebars.render("video_page", &video_data)?;

        fs::write(format!("build/video/{}.html", video.short_name), rendered)?;
    }

    let _ = fs::remove_dir_all("build/tag");
    fs::create_dir_all("build/tag")?;
    for tag in &home_data.all_tags {
        let tag_data = TagPageData {
            selected_tag: &tag.name,
            page_title: format!("Vidéos Lindy Hop - {}", tag.name),
            build_time: home_data.build_time,
            access_salt: home_data.access_salt,
            access_iterations: home_data.access_iterations,
            videos: home_data
                .videos
                .iter()
                .filter(|video| {
                    video
                        .tags
                        .iter()
                        .any(|video_tag| video_tag.name == tag.name)
                })
                .collect(),
        };

        let rendered = handlebars.render("tag_page", &tag_data)?;

        fs::write(format!("build/tag/{}.html", tag.clean_name), rendered)?;
    }

    let rendered = handlebars.render("about_page", &())?;
    fs::write("build/a-propos.html", rendered)?;

    fs::write("build/css.css", asset_data("static/css.css")?)?;
    fs::write("build/js.mjs", asset_data("static/js.mjs")?)?;
    fs::write(
        "build/favicon.png",
        asset_binary_data("static/favicon.png")?,
    )?;

    Ok(())
}

fn handlebars() -> anyhow::Result<Handlebars<'static>> {
    let mut handlebars = Handlebars::new();

    handlebars.register_partial("head", asset_data("partials/head.html.hbs")?)?;
    handlebars.register_partial("video", asset_data("partials/video.html.hbs")?)?;
    handlebars.register_partial("search", asset_data("partials/search.html.hbs")?)?;

    handlebars.register_template_string("about_page", asset_data("pages/about_page.html.hbs")?)?;
    handlebars.register_template_string("home_page", asset_data("pages/home_page.html.hbs")?)?;
    handlebars.register_template_string("tag_page", asset_data("pages/tag_page.html.hbs")?)?;
    handlebars.register_template_string("video_page", asset_data("pages/video_page.html.hbs")?)?;

    Ok(handlebars)
}

fn asset_binary_data(name: &str) -> anyhow::Result<Vec<u8>> {
    let file = Asset::get(name).with_context(|| format!("missing static file {}", name))?;
    Ok(file.data.into_owned())
}

fn asset_data(name: &str) -> anyhow::Result<String> {
    let data = String::from_utf8(asset_binary_data(name)?)?;
    Ok(data)
}

fn home_page_data<'a>(
    config: &'a Config,
    library: &'a Library,
) -> anyhow::Result<HomePageData<'a>> {
    let mut videos = Vec::with_capacity(library.videos.len());

    for library_video in &library.videos {
        let restriction = library_video.restriction.as_ref();
        let short_name = &library_video.video[0..config.thumbnail_hex_chars_prefix];
        let tags = library_video
            .tags
            .iter()
            .map(|tag| TagData {
                name: tag.clone(),
                clean_name: unidecode(tag)
                    .chars()
                    .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
                    .collect(),
            })
            .collect_vec();
        let template_video = VideoData {
            tags,
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

    let all_tags = videos
        .iter()
        .flat_map(|video| &video.tags)
        .unique()
        .cloned()
        .collect_vec();

    let build_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    Ok(HomePageData {
        build_time,
        all_tags,
        access_salt: &config.file_access_salt,
        access_iterations: config.file_access_iterations,
        videos,
        thumbnail_height: config.thumbnail_height,
    })
}

#[derive(Embed)]
#[folder = "web_src"]
struct Asset;

#[derive(Debug, Serialize)]
struct HomePageData<'a> {
    build_time: u64,
    access_salt: &'a str,
    access_iterations: u32,
    videos: Vec<VideoData<'a>>,
    all_tags: Vec<TagData>,
    thumbnail_height: u32,
}

#[derive(Debug, Serialize)]
struct VideoPageData<'a> {
    public_url: &'a str,
    page_title: String,
    build_time: u64,
    access_salt: &'a str,
    access_iterations: u32,
    video: &'a VideoData<'a>,
}

#[derive(Debug, Serialize)]
struct TagPageData<'a> {
    selected_tag: &'a str,
    page_title: String,
    build_time: u64,
    access_salt: &'a str,
    access_iterations: u32,
    videos: Vec<&'a VideoData<'a>>,
}

#[derive(Debug, Serialize)]
struct VideoData<'a> {
    tags: Vec<TagData>,
    short_name: &'a str,
    thumbnail: String,
    video: Option<&'a str>,
    access_rule: Option<&'a str>,
    access_iv: Option<&'a str>,
    access_ciphertext: Option<&'a str>,
    share_link: String,
}

#[derive(Debug, Serialize, Eq, PartialEq, Clone, Ord, PartialOrd, Hash)]
struct TagData {
    name: String,
    clean_name: String,
}
