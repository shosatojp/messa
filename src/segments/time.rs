use crate::builder::*;
use crate::util::colors::RawAppearance;
use crate::util::*;
use chrono::Local;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RawTimeConfig {
    pub appearance: RawAppearance,
    #[serde(default = "default_format_long")]
    pub format_long: String,
    #[serde(default = "default_format_medium")]
    pub format_medium: String,
    #[serde(default = "default_format_short")]
    pub format_short: String,
}

fn default_format_long() -> String {
    "%Y/%m/%d %H:%M:%S".to_string()
}

fn default_format_medium() -> String {
    "%H:%M:%S".to_string()
}

fn default_format_short() -> String {
    "%H:%M".to_string()
}

pub struct Time {
    enabled: bool,
    config: RawTimeConfig,
    pub size: [u32; 3],
}

impl Time {
    pub fn new(config: &RawTimeConfig) -> Time {
        let mut time = Time {
            enabled: true,
            config: config.clone(),
            size: [0, 0, 0],
        };

        if time.enabled {
            time.size[2] = time.construct(LengthLevel::LONG, BuildMode::ESTIMATE).count as u32;
            time.size[1] = time
                .construct(LengthLevel::MEDIUM, BuildMode::ESTIMATE)
                .count as u32;
            time.size[0] = time
                .construct(LengthLevel::SHORT, BuildMode::ESTIMATE)
                .count as u32;
        }
        return time;
    }
}

impl PromptSegment for Time {
    fn construct(&self, level: LengthLevel, mode: BuildMode) -> PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);
        builder.push(' ');
        match level {
            LengthLevel::LONG => {
                builder.push_string(&Local::now().format(&self.config.format_long).to_string())
            }
            LengthLevel::MEDIUM => {
                builder.push_string(&Local::now().format(&self.config.format_medium).to_string())
            }
            LengthLevel::SHORT => {
                builder.push_string(&Local::now().format(&self.config.format_short).to_string())
            }
        }
        builder.push(' ');
        return builder;
    }
    fn get_size(&self) -> &[u32; 3] {
        return &self.size;
    }
    fn get_fg(&self) -> String {
        return self.config.appearance.get_fg().to_string();
    }
    fn get_bg(&self) -> String {
        return self.config.appearance.get_bg().to_string();
    }
    fn is_enabled(&self) -> bool {
        return self.enabled;
    }
}
