use socialvoid_types::*;

use chrono::{DateTime, Utc};
use std::time::{Duration, UNIX_EPOCH};

pub struct SVPost(Post);
pub struct SVProfile(Profile);

impl std::convert::From<Post> for SVPost {
    fn from(post: Post) -> Self {
        Self(post)
    }
}

impl std::convert::From<Profile> for SVProfile {
    fn from(profile: Profile) -> Self {
        Self(profile)
    }
}

impl std::fmt::Display for SVPost {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Post Type: {}
ID: {}
Author: {}
Source: {}
----
{}
----
{} attachment(s)
Posted on: {}
Likes: {}, Reposts: {}, Quotes: {}, Replies: {},
Flags: ",
            match self.0.post_type {
                PostType::Reply => format!(
                    "Reply to <{}>",
                    match &self.0.reply_to_post {
                        Some(reply_to_post) => reply_to_post.id.clone(),
                        None => String::new(),
                    }
                ),
                PostType::Quote => format!(
                    "Quoted post <{}>",
                    match &self.0.quoted_post.as_ref() {
                        Some(quoted_post) => quoted_post.id.clone(),
                        None => String::new(),
                    }
                ),
                PostType::Repost => format!(
                    "Reposted post <{}>",
                    match &self.0.reposted_post.as_ref() {
                        Some(reposted_post) => reposted_post.id.clone(),
                        None => String::new(),
                    }
                ),
                _ => format!("{:?}", self.0.post_type),
            },
            self.0.id,
            self.0
                .peer
                .as_ref()
                .map(|x| x.username.to_string())
                .unwrap_or_else(|| "<unavailable>".to_string()),
            self.0
                .source
                .as_ref()
                .unwrap_or(&"<not applicable>".to_owned()),
            self.0.text.as_ref().unwrap_or(&"<no text>".to_string()),
            self.0.attachments.len(), //TODO: maybe show the document IDs
            {
                let d = UNIX_EPOCH + Duration::from_secs(self.0.posted_timestamp);
                // Create DateTime from SystemTime
                let datetime = DateTime::<Utc>::from(d);
                // Formats the combined date and time with the specified format string.
                datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string()
            },
            self.0
                .like_count
                .map(|x| x.to_string())
                .unwrap_or_else(|| "<not applicable>".to_string()),
            self.0
                .repost_count
                .map(|x| x.to_string())
                .unwrap_or_else(|| "<not applicable>".to_string()),
            self.0
                .quote_count
                .map(|x| x.to_string())
                .unwrap_or_else(|| "<not applicable>".to_string()),
            self.0
                .reply_count
                .map(|x| x.to_string())
                .unwrap_or_else(|| "<not applicable>".to_string()),
        )
    }
}

impl std::fmt::Display for SVProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "First Name: {}
{}
Name: {}
{}
{}
{}
Followers: {}
Following: {}
Display Picture: {}",
            self.0.first_name,
            self.0
                .last_name
                .as_ref()
                .map(|x| format!("Last Name: {}", x))
                .unwrap_or_else(|| String::from("[No last name set]")),
            self.0.name,
            self.0
                .biography
                .as_ref()
                .map(|x| format!("Biography: {}", x))
                .unwrap_or_else(|| String::from("[No biography set]")),
            self.0
                .location
                .as_ref()
                .map(|x| format!("Location: {}", x))
                .unwrap_or_else(|| String::from("[No location set]")),
            self.0
                .url
                .as_ref()
                .map(|x| format!("URL: {}", x))
                .unwrap_or_else(|| String::from("[No URL set]")),
            self.0.followers_count,
            self.0.following_count,
            if self.0.display_picture_sizes.is_empty() {
                String::from("not set")
            } else {
                let count = self.0.display_picture_sizes.len();
                let name = &self.0.display_picture_sizes[0].document.file_name;
                format!("'{}' ({} sizes available)", name, count)
            }
        )
    }
}
