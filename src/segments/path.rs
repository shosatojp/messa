use crate::builder::*;
use crate::util::colors::RawAppearance;
use crate::util::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RawPathConfig {
    pub appearance: RawAppearance,
}
pub struct Path {
    home: String,
    pwd: String,
    fg: String,
    bg: String,
    pub size: [u32; 3],
}

impl Path {
    pub fn new(config: &RawPathConfig, home: &str, pwd: &str) -> Path {
        let mut path = Path {
            fg: config.appearance.get_fg(),
            bg: config.appearance.get_bg(),
            home: home.to_owned(),
            pwd: pwd.to_owned(),
            size: [0, 0, 0],
        };

        path.size[2] = path.construct(LengthLevel::LONG, BuildMode::ESTIMATE).count as u32;
        path.size[1] = path
            .construct(LengthLevel::MEDIUM, BuildMode::ESTIMATE)
            .count as u32;
        path.size[0] = path
            .construct(LengthLevel::SHORT, BuildMode::ESTIMATE)
            .count as u32;
        return path;
    }
}

impl PromptSegment for Path {
    fn construct(&self, level: LengthLevel, mode: BuildMode) -> PromptStringBuilder {
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
