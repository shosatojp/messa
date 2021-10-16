use crate::builder::*;
use crate::util::*;
use chrono::Local;

pub struct Time {
    enabled: bool,
    fg: String,
    bg: String,
    pub size: [u32; 3],
}

impl Time {
    pub fn new(fg: &str, bg: &str) -> Time {
        let mut time = Time {
            enabled: true,
            fg: fg.to_string(),
            bg: bg.to_string(),
            size: [0, 0, 0],
        };

        if time.enabled {
            time.size[2] = time
                .construct(LengthLevel::LONG, BuildMode::ESTIMATE)
                .count as u32;
            time.size[1] = time.size[2];
            time.size[0] = time
                .construct(LengthLevel::SHORT, BuildMode::ESTIMATE)
                .count as u32;
        }
        return time;
    }
}

impl PromptSegment for Time {
    fn construct(&self, _level: LengthLevel, mode: BuildMode) -> PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);
        builder.push(' ');
        builder.push_string(&Local::now().format("%H:%M:%S").to_string());
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
        return self.enabled;
    }
}
