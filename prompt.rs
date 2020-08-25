use super::builder::*;
use super::lib::colors::*;
use super::lib::symbols::*;
use super::lib::*;
use git2::{Branch, Repository};

pub struct Prompt {
    prev_error: i8,
    fg: &'static str,
    bg: &'static str,
    size: [u32; 3],
}

impl Prompt {
    pub fn new(fg: &'static str, bg: &'static str, prev_error: i8) -> Prompt {
        let mut prompt = Prompt {
            fg,
            bg,
            prev_error,
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

impl PartialPrompt for Prompt {
    fn construct(&self, level: LENGTH_LEVEL, mode: BuildMode) -> PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);
        builder.push_string(&background(self.bg));
        builder.push_string(&forground(self.fg));
        builder.push_string(&format!(
            "🤗 {} $ ",
            if self.prev_error > 0 {
                self.prev_error.to_string()
            } else {
                "".to_string()
            }
        ));
        builder.push_string(&forground(self.bg));
        builder.push_string(&resetbackground());
        builder.push(SYMBOL_RIGHT);
        builder.push_string(&resetcolor());
        return builder;
    }
    fn get_size(&self) -> &[u32; 3] {
        return &self.size;
    }
    fn get_fg(&self) -> &str {
        return self.fg;
    }
    fn get_bg(&self) -> &str {
        return self.bg;
    }
}
