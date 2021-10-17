use crate::builder::{BuildMode, PromptStringBuilder};
use crate::util::{LengthLevel, PromptSegment, symbols::*};
use crate::{util::colors::RawAppearance};
use git2::{Branch, Repository, StatusOptions};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct RawGitConfig {
    pub appearance: RawAppearance,
    #[serde(default = "return_true")]
    pub count_unpushed: bool,
    #[serde(default = "return_true")]
    pub show_status: bool,
}

fn return_true() -> bool {
    true
}

pub struct Git {
    enabled: bool,
    branch_name: String,
    changed: u32,
    staged: u32,
    unpushed: u32,
    config: RawGitConfig,
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
                    branch_name = branch.name().unwrap_or(None).unwrap_or("").to_string();
                    if config.count_unpushed {
                        unpushed = count_unpushed(&repo, &branch).unwrap_or(0);
                    } else {
                        unpushed = has_unpushed(branch).unwrap_or(false) as u32;
                    }
                }
                let (changed, staged) = match config.show_status {
                    true => count_git_status(&repo),
                    false => (0, 0),
                };

                Git {
                    enabled: true,
                    branch_name,
                    changed,
                    staged,
                    unpushed,
                    config: config.clone(),
                    size: [0, 0, 0],
                }
            }
            None => Git {
                enabled: false,
                branch_name: "".to_string(),
                changed: 0,
                staged: 0,
                unpushed: 0,
                config: config.clone(),
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
                if self.config.show_status {
                    if self.changed > 0 {
                        builder.push(SYMBOL_GIT_CHANGED);
                    }
                    if self.staged > 0 {
                        builder.push(SYMBOL_GIT_STAGED);
                    }
                }
                if self.unpushed > 0 {
                    if self.config.count_unpushed {
                        builder.push_string(&format!(" {}{}", SYMBOL_GIT_UNPUSHED, self.unpushed));
                    } else {
                        builder.push_string(&format!(" {}", SYMBOL_GIT_UNPUSHED));
                    }
                }
            }

            builder.push(' ');
        }
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

pub fn count_git_status(repo: &Repository) -> (u32, u32) {
    let staged_mask = 0b11111;
    let changed_mask = 0b11111 << 7;

    let mut changed = false;
    let mut staged = false;
    let mut options = StatusOptions::default();

    for status in repo.statuses(Some(&mut options)).unwrap().iter() {
        let bits = &status.status().bits();
        changed |= bits & changed_mask > 0;
        staged |= bits & staged_mask > 0;
        if staged && changed {
            break;
        }
    }

    return (changed as u32, staged as u32);
}

pub fn count_unpushed(repo: &Repository, branch: &Branch) -> Result<u32, &'static str> {
    let mut rw = repo.revwalk().or(Err("could not get revwalk"))?;
    rw.push_head().or(Err("could not push head"))?;
    let upstream = branch.upstream().or(Err("could not get upstream"))?;
    let oid = upstream
        .into_reference()
        .target()
        .ok_or("could not get oid")?;
    rw.hide(oid).or(Err("could not hide upstream oid"))?;

    return Ok(rw.count() as u32);
}

pub fn has_unpushed(branch: Branch) -> Result<bool, String> {
    let origin = branch.upstream().or(Err("failed to get upstream"))?;
    let remote_head_oid = match origin.into_reference().target() {
        Some(oid) => oid,
        None => Err("failed to get remote head oid".to_string())?,
    };

    let local_head_oid = match branch.into_reference().target() {
        Some(oid) => oid,
        None => Err("failed to get local head oid".to_string())?,
    };

    Ok(remote_head_oid != local_head_oid)
}
