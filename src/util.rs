use std::process::exit;

use crate::builder::*;
use git2::{Branch, Repository, StatusOptions};

pub mod colors {
    use serde::Deserialize;
    use std::process::exit;

    pub const RED: &str = "5;203";
    pub const PINK: &str = "5;161";
    pub const PURPLE: &str = "5;127";
    pub const DEEP_PURPLE: &str = "5;61";
    pub const INDIGO: &str = "5;61";
    pub const BLUE: &str = "5;33";
    pub const LIGHT_BLUE: &str = "5;39";
    pub const CYAN: &str = "5;38";
    pub const TEAL: &str = "5;30";
    pub const GREEN: &str = "5;71";
    pub const LIGHT_GREEN: &str = "5;107";
    pub const LIME: &str = "5;107";
    pub const YELLOW: &str = "5;221";
    pub const AMBER: &str = "5;214";
    pub const ORANGE: &str = "5;208";
    pub const DEEP_ORANGE: &str = "5;202";
    pub const BROWN: &str = "5;";
    pub const GREY: &str = "5;247";
    pub const BLUE_GREY: &str = "5;66";
    pub const WHITE: &str = "5;15";
    pub const BLACK: &str = "5;0";
    pub fn forground(color: &str) -> String {
        return format!("\\[\x1b[38;{}m\\]", color);
    }

    pub fn background(color: &str) -> String {
        return format!("\\[\x1b[48;{}m\\]", color);
    }

    pub fn resetbackground() -> String {
        return String::from("\\[\x1b[49;24m\\]");
    }

    pub fn resetcolor() -> String {
        return String::from("\\[\x1b[0m\\]");
    }

    pub fn from_humanreadable(color_string: &str) -> &str {
        match color_string.to_uppercase().as_str() {
            "RED" => RED,
            "PINK" => PINK,
            "PURPLE" => PURPLE,
            "DEEP_PURPLE" => DEEP_PURPLE,
            "INDIGO" => INDIGO,
            "BLUE" => BLUE,
            "LIGHT_BLUE" => LIGHT_BLUE,
            "CYAN" => CYAN,
            "TEAL" => TEAL,
            "GREEN" => GREEN,
            "LIGHT_GREEN" => LIGHT_GREEN,
            "LIME" => LIME,
            "YELLOW" => YELLOW,
            "AMBER" => AMBER,
            "ORANGE" => ORANGE,
            "DEEP_ORANGE" => DEEP_ORANGE,
            "BROWN" => BROWN,
            "GREY" => GREY,
            "BLUE_GREY" => BLUE_GREY,
            "WHITE" => WHITE,
            "BLACK" => BLACK,
            _ => {
                eprintln!("unsupported color: {}", color_string);
                exit(1);
            }
        }
    }

    pub fn from_color_config(color_config: &str) -> String {
        match color_config.parse::<u8>() {
            Ok(code) => format!("5;{}", code),
            Err(_) => from_humanreadable(color_config).to_string(),
        }
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct RawAppearance {
        pub fg: String,
        pub bg: String,
    }

    impl RawAppearance {
        pub fn get_fg(&self) -> String {
            return from_color_config(&self.fg);
        }
        pub fn get_bg(&self) -> String {
            return from_color_config(&self.bg);
        }
    }
}

pub mod symbols {
    pub const SYMBOL_RIGHT: char = '\u{e0b0}';
    pub const SYMBOL_LEFT: char = '\u{e0b2}';
    pub const SYMBOL_RIGHT_ALT: char = '\u{e0b1}';
    pub const SYMBOL_GIT_UNPUSHED: char = '↑';
    pub const SYMBOL_GIT_BRANCH: char = '\u{e0a0}';
    pub const SYMBOL_GIT_CHANGED: char = '\x2a';
    pub const SYMBOL_GIT_STAGED: char = '\x2b';
    pub const SYMBOL_SSH: char = '🌐';
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Location {
    LEFT,
    RIGHT,
}

pub fn load_location(location: &str) -> Location {
    match location.to_uppercase().as_str() {
        "LEFT" => Location::LEFT,
        "RIGHT" => Location::RIGHT,
        _ => {
            eprintln!("Unsupported location: {}", &location);
            exit(1);
        }
    }
}

pub trait PromptSegment {
    fn construct(&self, level: LengthLevel, mode: BuildMode) -> PromptStringBuilder;
    fn get_size(&self) -> &[u32; 3];
    fn get_fg(&self) -> String;
    fn get_bg(&self) -> String;
    fn is_enabled(&self) -> bool;
}

pub fn build_path_str(home_src: &str, path_src: &str, level: LengthLevel) -> String {
    let home = home_src.as_bytes();
    let home_len = home.len();
    let path = path_src.as_bytes();
    let mut in_home = false;

    let mut slice_start = 0;

    if path.len() >= home_len {
        for i in 0..home_len {
            if path[i] != home[i] {
                break;
            }
            if i + 1 == home_len {
                in_home = true;
                slice_start = i + 1;
            }
        }
    }
    match level {
        LengthLevel::LONG => {
            let mut piecies: Vec<String> = vec![];
            if in_home {
                piecies.push("~".to_string());
            } else {
                piecies.push("/".to_string());
            }
            for piece in path_src[slice_start..].split('/') {
                if piece.len() > 0 {
                    piecies.push(piece.to_string());
                }
            }
            return piecies.join(format!(" {} ", symbols::SYMBOL_RIGHT_ALT).as_str());
        }
        LengthLevel::MEDIUM => {
            let mut sliced = path_src[slice_start..].to_string();
            if !sliced.starts_with("/") {
                sliced.insert(0, '/');
            }
            if in_home {
                sliced.insert(0, '~');
            }
            return sliced;
        }
        LengthLevel::SHORT => {
            return path_src[slice_start..]
                .split('/')
                .last()
                .unwrap()
                .to_string();
        }
    }
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum LengthLevel {
    LONG = 2,
    MEDIUM = 1,
    SHORT = 0,
}

pub fn load_lengthlevel(lengthlevel: &str) -> LengthLevel {
    match lengthlevel.to_uppercase().as_str() {
        "LONG" => LengthLevel::LONG,
        "MEDIUM" => LengthLevel::MEDIUM,
        "SHORT" => LengthLevel::SHORT,
        _ => {
            eprintln!("Unsupported length level: {}", &lengthlevel);
            exit(1);
        }
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

pub fn expand_user(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    if path.starts_with("~") {
        let home_dir = std::env::var("HOME")?;
        if path.starts_with("~/") {
            let p = std::path::Path::new(&home_dir);
            let joined = p.join(path.strip_prefix("~/").unwrap());
            return Ok(joined.to_str().unwrap().to_string());
        } else {
            return Ok(home_dir);
        }
    } else {
        return Ok(path.to_string());
    }
}
