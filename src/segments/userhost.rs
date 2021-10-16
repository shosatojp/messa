use crate::builder::*;
use crate::util::colors::RawAppearance;
use crate::util::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RawUserhostConfig {
    pub appearance: RawAppearance,
}

pub struct UserHostname {
    appearance: RawAppearance, 
    username: String,
    hostname: String,
    pub size: [u32; 3],
}

impl UserHostname {
    pub fn new(config: &RawUserhostConfig, user: &str, host: &str) -> UserHostname {
        let mut userhost = UserHostname {
            username: user.to_string(),
            hostname: host.to_string(),
            appearance: config.appearance.clone(),
            size: [0, 0, 0],
        };

        userhost.size[2] = userhost
            .construct(LengthLevel::LONG, BuildMode::ESTIMATE)
            .count as u32;
        userhost.size[1] = userhost.size[2];
        userhost.size[0] = userhost
            .construct(LengthLevel::SHORT, BuildMode::ESTIMATE)
            .count as u32;
        return userhost;
    }
}

impl PromptSegment for UserHostname {
    fn construct(&self, level: LengthLevel, mode: BuildMode) -> PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);

        builder.push(' ');
        builder.push_string(&self.username);

        if level >= LengthLevel::MEDIUM {
            builder.push('@');
            builder.push_string(&self.hostname);
        }
        builder.push(' ');

        return builder;
    }
    fn get_size(&self) -> &[u32; 3] {
        return &self.size;
    }
    fn get_fg(&self) -> String {
        return self.appearance.get_fg().to_string();
    }
    fn get_bg(&self) -> String {
        return self.appearance.get_bg().to_string();
    }
    fn is_enabled(&self) -> bool {
        return true;
    }
}
