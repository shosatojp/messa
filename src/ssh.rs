use super::builder::*;
use super::util::colors::*;
use super::util::symbols::*;
use super::util::*;
use git2::{Branch, Repository};

pub struct Ssh {
    enabled: bool,
    fg: &'static str,
    bg: &'static str,
    pub size: [u32; 3],
}

impl Ssh {
    pub fn new<'a>(fg: &'static str, bg: &'static str) -> Ssh {
        let mut ssh = Ssh {
            enabled: std::env::var("SSH_TTY")
                .and_then(|s| Ok(s.len()))
                .unwrap_or(0)
                != 0,
            fg,
            bg,
            size: [0, 0, 0],
        };

        if ssh.enabled {
            ssh.size[2] = ssh.construct(LENGTH_LEVEL::LONG, BuildMode::ESTIMATE).count as u32;
            ssh.size[1] = ssh.size[2];
            ssh.size[0] = ssh
                .construct(LENGTH_LEVEL::SHORT, BuildMode::ESTIMATE)
                .count as u32;
        }
        return ssh;
    }
}

impl PromptSegment for Ssh {
    fn construct(&self, level: LENGTH_LEVEL, mode: BuildMode) -> PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);
        builder.push(' ');
        if level >= LENGTH_LEVEL::MEDIUM {
            builder.push(SYMBOL_SSH);
            builder.count += 1; // SYMBOL_SSH が幅２
            builder.push(' ');
        }
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
    fn is_enabled(&self) -> bool {
        return self.enabled;
    }
}
