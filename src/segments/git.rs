use crate::util::symbols::*;
use crate::util::*;
use crate::{builder::*, util::colors::RawAppearance};
use git2::{Branch, Repository};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RawGitConfig {
    pub appearance: RawAppearance,
}

pub struct Git {
    enabled: bool,
    branch_name: String,
    changed: u32,
    staged: u32,
    unpushed: u32,
    fg: String,
    bg: String,
    pub size: [u32; 3],
}

impl Git {
    pub fn new(config: &RawGitConfig, pwd: &str) -> Git {
        let mut repo = None;
        for parent in std::path::Path::new(pwd)
            .ancestors()
            .filter(|&path| path.join(".git").exists())
        {
            repo = match Repository::open(parent) {
                Ok(repo) => Some(repo),
                Err(_) => continue,
            }
        }
        let mut git: Git = match repo {
            Some(repo) => {
                let mut unpushed = 0;
                let mut branch_name = String::new();
                let head = repo.head();
                if head.is_ok() {
                    let branch = Branch::wrap(head.unwrap());
                    unpushed = count_unpushed(&repo, &branch).unwrap_or(0);
                    branch_name = branch.name().unwrap_or(None).unwrap_or("").to_string()
                }
                let (changed, staged) = count_git_status(&repo);

                Git {
                    enabled: true,
                    branch_name,
                    changed,
                    staged,
                    unpushed,
                    fg: config.appearance.get_fg(),
                    bg: config.appearance.get_bg(),
                    size: [0, 0, 0],
                }
            }
            None => Git {
                enabled: false,
                branch_name: "".to_string(),
                changed: 0,
                staged: 0,
                unpushed: 0,
                fg: config.appearance.get_fg(),
                bg: config.appearance.get_bg(),
                size: [0, 0, 0],
            },
        };

        git.size[2] = git.construct(LengthLevel::LONG, BuildMode::ESTIMATE).count as u32;
        git.size[1] = git
            .construct(LengthLevel::MEDIUM, BuildMode::ESTIMATE)
            .count as u32;
        git.size[0] = git.construct(LengthLevel::SHORT, BuildMode::ESTIMATE).count as u32;
        return git;
    }
}

impl PromptSegment for Git {
    fn construct(&self, level: LengthLevel, mode: BuildMode) -> PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);
        if self.enabled {
            if level >= LengthLevel::MEDIUM {
                builder.push(' ');
                builder.push(SYMBOL_GIT_BRANCH);
                if level >= LengthLevel::LONG {
                    if self.branch_name.len() > 0 {
                        builder.push_string(&format!(" {}", self.branch_name));
                    }
                }
                if self.changed > 0 {
                    builder.push(SYMBOL_GIT_CHANGED);
                }
                if self.staged > 0 {
                    builder.push(SYMBOL_GIT_STAGED);
                }
                if self.unpushed > 0 {
                    builder.push_string(&format!(" {}{}", SYMBOL_GIT_UNPUSHED, self.unpushed));
                }
            }

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
