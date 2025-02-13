use crate::tags_file::TagsVideo;
use serde::Deserialize;

/// Declare all the access rules used to restrict the visibility of the videos
#[derive(Debug, Deserialize, Default)]
pub struct Restrictions {
    pub rules: Vec<RestrictionRule>,
}

#[derive(Debug, Deserialize)]
pub struct RestrictionRule {
    pub name: String,
    #[serde(default)]
    pub with_tags: Vec<String>,
    #[serde(default)]
    pub without_tags: Vec<String>,
    pub password: String,
}

impl Restrictions {
    pub fn find(&self, video: &TagsVideo) -> Option<&RestrictionRule> {
        self.rules.iter().find(|rule| rule.matches(video))
    }
}

impl RestrictionRule {
    fn matches(&self, video: &TagsVideo) -> bool {
        self.with_tags.iter().all(|tag| video.tags.contains(tag))
            && self
                .without_tags
                .iter()
                .all(|tag| !video.tags.contains(tag))
    }
}
