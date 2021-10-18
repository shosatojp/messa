use std::process::exit;

use crate::builder::*;

type ColorCode = u8;

pub mod colors {
    use serde::Deserialize;
    use std::process::exit;

    use super::ColorCode;

    pub const RED: ColorCode = 203;
    pub const PINK: ColorCode = 161;
    pub const PURPLE: ColorCode = 127;
    pub const DEEP_PURPLE: ColorCode = 61;
    pub const INDIGO: ColorCode = 61;
    pub const BLUE: ColorCode = 33;
    pub const LIGHT_BLUE: ColorCode = 39;
    pub const CYAN: ColorCode = 38;
    pub const TEAL: ColorCode = 30;
    pub const GREEN: ColorCode = 71;
    pub const LIGHT_GREEN: ColorCode = 107;
    pub const LIME: ColorCode = 107;
    pub const YELLOW: ColorCode = 221;
    pub const AMBER: ColorCode = 214;
    pub const ORANGE: ColorCode = 208;
    pub const DEEP_ORANGE: ColorCode = 202;
    pub const BROWN: ColorCode = 0;
    pub const GREY: ColorCode = 247;
    pub const BLUE_GREY: ColorCode = 66;
    pub const WHITE: ColorCode = 15;
    pub const BLACK: ColorCode = 0;

    pub fn from_humanreadable(color_string: &str) -> u8 {
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

    pub fn color_code(config: &str) -> ColorCode {
        match config.parse::<u8>() {
            Ok(code) => code,
            Err(_) => from_humanreadable(config),
        }
    }

    #[derive(Deserialize, Debug, Clone)]
    pub struct RawAppearance {
        pub fg: String,
        pub bg: String,
    }

    impl RawAppearance {
        pub fn get_fg(&self) -> String {
            return self.fg.to_string();
        }
        pub fn get_bg(&self) -> String {
            return self.bg.to_string();
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
    PROMPT,
}

pub fn load_location(location: &str) -> Location {
    match location.to_uppercase().as_str() {
        "LEFT" => Location::LEFT,
        "RIGHT" => Location::RIGHT,
        "PROMPT" => Location::PROMPT,
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

pub fn expand_user(home_dir: &str, path: &str) -> String {
    if path.starts_with("~") {
        if path.starts_with("~/") {
            let p = std::path::Path::new(&home_dir);
            let joined = p.join(path.strip_prefix("~/").unwrap());
            return joined.to_str().unwrap().to_string();
        } else {
            return home_dir.to_string();
        }
    } else {
        return path.to_string();
    }
}
