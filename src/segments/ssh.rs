use crate::builder::*;
use crate::util::colors::RawAppearance;
use crate::util::symbols::*;
use crate::util::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RawSshConfig {
    pub appearance: RawAppearance,
}

pub struct Ssh {
    enabled: bool,
    fg: String,
    bg: String,
    pub size: [u32; 3],
}

impl Ssh {
    pub fn new<'a>(config: &RawSshConfig) -> Ssh {
        let mut ssh = Ssh {
            enabled: std::env::var("SSH_TTY")
                .and_then(|s| Ok(s.len()))
                .unwrap_or(0)
                != 0,
            fg: config.appearance.get_fg(),
            bg: config.appearance.get_bg(),
            size: [0, 0, 0],
        };

        if ssh.enabled {
            ssh.size[2] = ssh.construct(LengthLevel::LONG, BuildMode::ESTIMATE).count as u32;
            ssh.size[1] = ssh.size[2];
            ssh.size[0] = ssh.construct(LengthLevel::SHORT, BuildMode::ESTIMATE).count as u32;
        }
        return ssh;
    }
}

impl PromptSegment for Ssh {
    fn construct(&self, level: LengthLevel, mode: BuildMode) -> PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);
        builder.push(' ');
        if level >= LengthLevel::MEDIUM {
            builder.push(SYMBOL_SSH);
            // builder.count += 1; // SYMBOL_SSH が幅２
            builder.push(' ');
        }
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
