use super::builder::*;
use super::util::colors::*;
use super::util::symbols::*;
use super::util::*;
use git2::{Branch, Repository};

pub struct Path<'a> {
    home: &'a str,
    pwd: &'a str,
    fg: &'static str,
    bg: &'static str,
    pub size: [u32; 3],
}

impl Path<'_> {
    pub fn new<'a>(fg: &'static str, bg: &'static str, home: &'a str, pwd: &'a str) -> Path<'a> {
        let mut path = Path {
            fg,
            bg,
            home,
            pwd,
            size: [0, 0, 0],
        };

        path.size[2] = path
            .construct(LENGTH_LEVEL::LONG, BuildMode::ESTIMATE)
            .count as u32;
        path.size[1] = path
            .construct(LENGTH_LEVEL::MEDIUM, BuildMode::ESTIMATE)
            .count as u32;
        path.size[0] = path
            .construct(LENGTH_LEVEL::SHORT, BuildMode::ESTIMATE)
            .count as u32;
        return path;
    }
}

impl PartialPrompt for Path<'_> {
    fn construct(&self, level: LENGTH_LEVEL, mode: BuildMode) -> PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);
        builder.push_string(&background(self.bg));
        builder.push(SYMBOL_RIGHT);
        builder.push_string(&forground(self.fg));
        builder.push(' ');
        builder.push_string(&build_path_str(self.home, self.pwd, level));
        builder.push(' ');
        builder.push_string(&forground(self.bg));
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
