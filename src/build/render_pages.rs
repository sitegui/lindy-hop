use crate::build::library::{Library, LibraryFile};
use anyhow::Context;
use handlebars::Handlebars;
use rust_embed::Embed;
use serde::Serialize;
use std::fs;

pub fn render_pages(library: &Library) -> anyhow::Result<()> {
    let handlebars = Handlebars::new();

    let mut videos = Vec::with_capacity(library.videos.len());

    for library_video in &library.videos {
        let (video, accesses) = match &library_video.file {
            LibraryFile::Public { video } => (Some(video.as_str()), None),
            LibraryFile::Private { accesses } => (None, Some(serde_json::to_string(accesses)?)),
        };

        videos.push(TemplateVideo {
            tags: &library_video.tags,
            thumbnail: &library_video.thumbnail,
            video,
            accesses,
        });
    }

    let template = Asset::get("index.html.hbs").context("missing template file")?;
    let template = std::str::from_utf8(&template.data)?;

    let rendered = handlebars.render_template(template, &TemplateData { videos })?;
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
    videos: Vec<TemplateVideo<'a>>,
}

#[derive(Debug, Serialize)]
struct TemplateVideo<'a> {
    tags: &'a [String],
    thumbnail: &'a str,
    video: Option<&'a str>,
    accesses: Option<String>,
}
