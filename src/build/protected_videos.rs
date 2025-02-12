use crate::tags_file::TagsVideo;
use serde::Deserialize;

/// Declare all the access rules used to restrict the visibility of the videos
#[derive(Debug, Deserialize)]
pub struct ProtectedVideos {
    pub rules: Vec<ProtectedVideosRule>,
}

#[derive(Debug, Deserialize)]
pub struct ProtectedVideosRule {
    pub name: String,
    #[serde(default)]
    pub with_tags: Vec<String>,
    #[serde(default)]
    pub without_tags: Vec<String>,
    pub password: String,
}

impl ProtectedVideos {
    pub fn find_rule(&self, video: &TagsVideo) -> Option<&ProtectedVideosRule> {
        self.rules.iter().find(|rule| rule.matches(video))
    }
}

impl ProtectedVideosRule {
    fn matches(&self, video: &TagsVideo) -> bool {
        self.with_tags.iter().all(|tag| video.tags.contains(tag))
            && self
                .without_tags
                .iter()
                .all(|tag| !video.tags.contains(tag))
    }
}
