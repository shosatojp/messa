use crate::builder::*;
use crate::util::colors::*;
use crate::util::symbols::*;
use crate::util::*;
use git2::{Branch, Repository};

pub struct UserHostname {
    fg: String,
    bg: String,
    username: String,
    hostname: String,
    pub size: [u32; 3],
}

impl UserHostname {
    pub fn new(fg: &str, bg: &str, user: &str, host: &str) -> UserHostname {
        let mut userhost = UserHostname {
            username: user.to_string(),
            hostname: host.to_string(),
            fg: fg.to_string(),
            bg: bg.to_string(),
            size: [0, 0, 0],
        };

        userhost.size[2] = userhost
            .construct(LENGTH_LEVEL::LONG, BuildMode::ESTIMATE)
            .count as u32;
        userhost.size[1] = userhost.size[2];
        userhost.size[0] = userhost
            .construct(LENGTH_LEVEL::SHORT, BuildMode::ESTIMATE)
            .count as u32;
        return userhost;
    }
}

impl PromptSegment for UserHostname {
    fn construct(&self, level: LENGTH_LEVEL, mode: BuildMode) -> PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);

        builder.push(' ');
        builder.push_string(&self.username);

        if level >= LENGTH_LEVEL::MEDIUM {
            builder.push('@');
            builder.push_string(&self.hostname);
        }
        builder.push(' ');

        return builder;
    }
    fn get_size(&self) -> &[u32; 3] {
        return &self.size;
    }
    fn get_fg(&self) -> &str {
        return &self.fg;
    }
    fn get_bg(&self) -> &str {
        return &self.bg;
    }
    fn is_enabled(&self) -> bool {
        return true;
    }
}
