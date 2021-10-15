use super::builder::*;
use super::util::colors::*;
use super::util::symbols::*;
use super::util::*;
use git2::{Branch, Repository};

pub struct Path {
    home: String,
    pwd: String,
    fg: String,
    bg: String,
    pub size: [u32; 3],
}

impl Path {
    pub fn new(fg: &str, bg: &str, home: &str, pwd: &str) -> Path {
        let mut path = Path {
            fg: fg.to_string(),
            bg: bg.to_string(),
            home: home.to_owned(),
            pwd: pwd.to_owned(),
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

impl PromptSegment for Path {
    fn construct(&self, level: LENGTH_LEVEL, mode: BuildMode) -> PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);
        builder.push(' ');
        builder.push_string(&build_path_str(
            self.home.as_str(),
            self.pwd.as_str(),
            level,
        ));
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
