use crate::builder::*;
use crate::util::colors::*;
use crate::util::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RawPromptConfig {
    pub ok: PromptStatusConfig,
    pub error: PromptStatusConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PromptStatusConfig {
    appearance: RawAppearance,
}

pub struct Prompt {
    prev_error: u8,
    config: RawPromptConfig,
    user: String,
    size: [u32; 3],
}

impl Prompt {
    pub fn new(config: &RawPromptConfig, user: &str, prev_error: u8) -> Prompt {
        let mut prompt = Prompt {
            config: config.clone(),
            prev_error,
            user: user.to_string(),
            size: [0, 0, 0],
        };

        let long = prompt
            .construct(LengthLevel::LONG, BuildMode::ESTIMATE)
            .count as u32;
        prompt.size[0] = long;
        prompt.size[1] = long;
        prompt.size[2] = long;

        return prompt;
    }
}

impl PromptSegment for Prompt {
    fn construct(&self, _level: LengthLevel, mode: BuildMode) -> PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);
        builder.push_string(&format!(
            " {} {} ",
            if self.prev_error > 0 {
                self.prev_error.to_string()
            } else {
                "".to_string()
            },
            if self.user == "root" { "#" } else { "$" }
        ));
        return builder;
    }
    fn get_size(&self) -> &[u32; 3] {
        return &self.size;
    }
    fn get_fg(&self) -> String {
        if self.prev_error == 0 {
            self.config.ok.appearance.get_fg()
        } else {
            self.config.error.appearance.get_fg()
        }
    }
    fn get_bg(&self) -> String {
        if self.prev_error == 0 {
            self.config.ok.appearance.get_bg()
        } else {
            self.config.error.appearance.get_bg()
        }
    }
    fn is_enabled(&self) -> bool {
        return true;
    }
}
