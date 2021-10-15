use super::builder::*;
use super::util::colors::*;
use super::util::symbols::*;
use super::util::*;
use git2::{Branch, Repository};

pub struct Prompt {
    prev_error: u8,
    fg: String,
    bg: String,
    user: String,
    size: [u32; 3],
}

impl Prompt {
    pub fn new(user: &str, fg: &str, bg: &str, prev_error: u8) -> Prompt {
        let mut prompt = Prompt {
            fg: fg.to_string(),
            bg: bg.to_string(),
            prev_error,
            user: user.to_string(),
            size: [0, 0, 0],
        };

        let long = prompt
            .construct(LENGTH_LEVEL::LONG, BuildMode::ESTIMATE)
            .count as u32;
        prompt.size[0] = long;
        prompt.size[1] = long;
        prompt.size[2] = long;

        return prompt;
    }
}

impl PromptSegment for Prompt {
    fn construct(&self, level: LENGTH_LEVEL, mode: BuildMode) -> PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);
        builder.push_string(&background(&self.bg));
        builder.push_string(&forground(&self.fg));
        builder.push_string(&format!(
            " {} {} ",
            if self.prev_error > 0 {
                self.prev_error.to_string()
            } else {
                "".to_string()
            },
            if self.user == "root" { "#" } else { "$" }
        ));
        builder.push_string(&forground(&self.bg));
        builder.push_string(&resetbackground());
        builder.push(SYMBOL_RIGHT);
        builder.push_string(&resetcolor());
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
