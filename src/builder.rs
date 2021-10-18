use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(PartialEq)]
pub enum BuildMode {
    ESTIMATE,
    CONSTRUCT,
}

pub struct PromptStringBuilder {
    pub data: String,
    pub count: usize,
    mode: BuildMode,
}

impl PromptStringBuilder {
    pub fn new(mode: BuildMode) -> PromptStringBuilder {
        return PromptStringBuilder {
            data: String::new(),
            count: 0,
            mode,
        };
    }
    pub fn push(&mut self, ch: char) {
        if self.mode == BuildMode::CONSTRUCT {
            self.data.push(ch);
        }
        self.count += UnicodeWidthChar::width(ch).unwrap_or(0);
    }
    pub fn push_string(&mut self, string: &str) {
        if self.mode == BuildMode::CONSTRUCT {
            self.data.push_str(string);
        }
        self.count += UnicodeWidthStr::width(string);
    }
    pub fn push_style(&mut self, style: &str) {
        if self.mode == BuildMode::CONSTRUCT {
            self.data.push_str(style);
        }
    }
}
