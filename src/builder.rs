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
    pub fn push_string(&mut self, string: &String) {
        if self.mode == BuildMode::CONSTRUCT {
            self.data.push_str(string.as_str());
        }
        if !string.starts_with("\\[\x1b") {
            self.count += UnicodeWidthStr::width(string.as_str());
            // self.count += string.chars().count();
        }
    }
    // fn push_str(&mut self, string: &str, len: usize) {
    //     if self.mode == BuildMode::CONSTRUCT {
    //         self.data.push_str(string);
    //     }
    //     if string.as_bytes()[0] != '\x1b' as u8 {
    //         self.count += len;
    //     }
    // }
}
