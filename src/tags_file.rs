//! Read and write to tags.txt format

use anyhow::Context;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct TagsFile {
    pub videos: Vec<TagsVideo>,
}

#[derive(Debug)]
pub struct TagsVideo {
    pub name: String,
    pub tags: Vec<String>,
}

impl TagsVideo {
    pub fn new(name: String) -> Self {
        TagsVideo {
            name,
            tags: Vec::new(),
        }
    }
}

impl FromStr for TagsFile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tags_file = TagsFile { videos: Vec::new() };
        let mut current_video = None;

        for line in s.lines() {
            let line = line.trim();

            if let Some(name) = line
                .strip_prefix('[')
                .and_then(|line| line.strip_suffix(']'))
            {
                let new_video = TagsVideo::new(name.trim().to_string());
                if let Some(video) = current_video.replace(new_video) {
                    tags_file.videos.push(video);
                }
            } else if !line.is_empty() {
                current_video
                    .as_mut()
                    .context("missing current_video")?
                    .tags
                    .push(line.to_string());
            }
        }

        if let Some(current_video) = current_video {
            tags_file.videos.push(current_video);
        }

        Ok(tags_file)
    }
}

impl Display for TagsFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, video) in self.videos.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }

            writeln!(f, "[{}]", video.name)?;
            for tag in &video.tags {
                writeln!(f, "{}", tag)?;
            }
        }

        Ok(())
    }
}
