use std::io::Write;
use std::process::exit;

use crate::builder::*;
use crate::config::{ProfileConfig, Segment, SegmentConfig};
use crate::shell::GenericShell;
use crate::util::symbols::*;
use crate::util::*;

pub fn out(width: u32, profiles: &Vec<ProfileConfig>, prompt: &Segment, shell: &GenericShell) {
    match out_line(width, profiles, shell) {
        Some(_profile) => {
            let segments: Vec<SegmentConfig> = vec![SegmentConfig {
                segment: prompt.clone(),
                location: Location::PROMPT,
                size: LengthLevel::LONG,
            }];
            out_prompt(&ProfileConfig { segments: segments }, shell)
        }
        None => {
            eprintln!("no suitable profile found");
            exit(1);
        }
    }
}

fn out_prompt(profile: &ProfileConfig, shell: &GenericShell) {
    let mut string = PromptStringBuilder::new(BuildMode::CONSTRUCT);
    for (i, seg) in profile
        .segments
        .iter()
        .filter(|seg| seg.segment.is_enabled() && seg.location == Location::PROMPT)
        .enumerate()
    {
        string.push_style(&shell.set_bg(&seg.segment.get_bg()));
        if i != 0 {
            string.push(SYMBOL_RIGHT);
        }
        string.push_style(&shell.set_fg(&seg.segment.get_fg()));
        string.push_string(
            (*seg.segment)
                .construct(seg.size, BuildMode::CONSTRUCT)
                .data
                .as_str(),
        );
        string.push_style(&shell.set_fg(&seg.segment.get_bg()));
    }
    string.push_style(&shell.resetbackground());
    string.push(SYMBOL_RIGHT);
    string.push_style(&shell.resetcolor());

    let _ = std::io::stdout().write_all(string.data.as_bytes());
    let _ = std::io::stdout().write_all(b" ");
}

fn out_line<'a>(
    width: u32,
    profiles: &'a Vec<ProfileConfig>,
    shell: &GenericShell,
) -> Option<&'a ProfileConfig> {
    for profile in profiles {
        let mut left_sum = 0;
        let mut right_sum = 0;

        for seg in &profile.segments {
            if seg.segment.is_enabled() {
                match seg.location {
                    Location::LEFT => left_sum += seg.segment.get_size()[seg.size as usize] + 1,
                    Location::RIGHT => right_sum += seg.segment.get_size()[seg.size as usize] + 1,
                    Location::PROMPT => continue,
                }
            }
        }

        let sum = left_sum + right_sum;

        if width >= sum {
            // left
            let mut string = PromptStringBuilder::new(BuildMode::CONSTRUCT);

            for (i, seg) in profile
                .segments
                .iter()
                .filter(|seg| seg.segment.is_enabled() && seg.location == Location::LEFT)
                .enumerate()
            {
                string.push_style(&shell.set_bg(&seg.segment.get_bg()));
                if i != 0 {
                    string.push(SYMBOL_RIGHT);
                }
                string.push_style(&shell.set_fg(&seg.segment.get_fg()));
                string.push_string(
                    (*seg.segment)
                        .construct(seg.size, BuildMode::CONSTRUCT)
                        .data
                        .as_str(),
                );
                string.push_style(&shell.set_fg(&seg.segment.get_bg()));
            }
            string.push_style(&shell.resetbackground());
            string.push(SYMBOL_RIGHT);
            string.push_style(&shell.resetcolor());

            // right
            let mut right_string = PromptStringBuilder::new(BuildMode::CONSTRUCT);

            for seg in profile
                .segments
                .iter()
                .filter(|seg| seg.segment.is_enabled() && seg.location == Location::RIGHT)
            {
                right_string.push_style(&shell.set_fg(&seg.segment.get_bg()));
                right_string.push(SYMBOL_LEFT);
                right_string.push_style(&shell.set_fg(&seg.segment.get_fg()));
                right_string.push_style(&shell.set_bg(&seg.segment.get_bg()));
                right_string.push_string(
                    seg.segment
                        .construct(seg.size, BuildMode::CONSTRUCT)
                        .data
                        .as_str(),
                );
            }
            right_string.push_style(&shell.resetcolor());

            string.push_string(" ".repeat(width as usize - sum as usize).as_str());
            string.push_string(&right_string.data);

            let _ = std::io::stdout().write_all(string.data.as_bytes());
            let _ = std::io::stdout().write_all(shell.newline().as_bytes());
            return Some(&profile);
        }
    }

    None
}
