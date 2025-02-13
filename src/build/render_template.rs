use crate::build::library::Library;
use anyhow::Context;
use handlebars::Handlebars;
use rust_embed::Embed;

pub fn render_template(library: &Library) -> anyhow::Result<String> {
    let handlebars = Handlebars::new();

    let template = Asset::get("index.html.hbs").context("missing template file")?;
    let template = std::str::from_utf8(&template.data)?;
    let rendered = handlebars.render_template(template, library)?;

    Ok(rendered)
}

#[derive(Embed)]
#[folder = "web_src"]
struct Asset;
