#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use git2::{Branch, Repository};
mod util;
use util::colors::*;
use util::symbols::*;
use util::*;
mod args;
use args::*;
mod path;
use path::*;
mod builder;
use builder::*;
mod userhost;
use userhost::*;
mod git;
use git::*;
mod ssh;
use ssh::*;
mod prompt;
use clap::ArgMatches;
use prompt::*;

fn main() -> Result<(), &'static str> {
    let matches: ArgMatches = get_arg_matches();

    // arguments
    let pwd = matches.value_of("pwd").ok_or("")?;
    let home = matches.value_of("home").unwrap();

    let width: u32 = matches.value_of("width").unwrap().parse().unwrap();
    let prev_error: u8 = matches.value_of("error").unwrap().parse().unwrap();

    // def colors
    let fg = WHITE;
    let bg_ssh = DEEP_PURPLE;
    let bg_user_hostname = INDIGO;
    let bg_path = TEAL;
    let bg_git = DEEP_ORANGE;
    let bg_prompt = if prev_error > 0 { PINK } else { CYAN };

    // partial prompt builders
    let segment_ssh: Box<dyn PromptSegment> = Box::new(Ssh::new(fg, bg_ssh));
    let segment_userhostname: Box<dyn PromptSegment> =
        Box::new(UserHostname::new(fg, bg_user_hostname));
    let segment_path: Box<dyn PromptSegment> = Box::new(Path::new(fg, bg_path, home, pwd));
    let segment_git: Box<dyn PromptSegment> = Box::new(Git::new(fg, bg_git, pwd));
    let prompt = Prompt::new(fg, bg_prompt, prev_error);

    // profiles
    let profiles: Vec<Vec<(&Box<dyn PromptSegment>, LENGTH_LEVEL)>> = vec![
        vec![
            (&segment_ssh, LENGTH_LEVEL::LONG),
            (&segment_userhostname, LENGTH_LEVEL::LONG),
            (&segment_path, LENGTH_LEVEL::LONG),
            (&segment_git, LENGTH_LEVEL::LONG),
        ],
        vec![
            (&segment_ssh, LENGTH_LEVEL::LONG),
            (&segment_userhostname, LENGTH_LEVEL::LONG),
            (&segment_path, LENGTH_LEVEL::MEDIUM),
            (&segment_git, LENGTH_LEVEL::LONG),
        ],
        vec![
            (&segment_ssh, LENGTH_LEVEL::LONG),
            (&segment_userhostname, LENGTH_LEVEL::LONG),
            (&segment_path, LENGTH_LEVEL::LONG),
            (&segment_git, LENGTH_LEVEL::LONG),
        ],
        vec![
            (&segment_ssh, LENGTH_LEVEL::LONG),
            (&segment_userhostname, LENGTH_LEVEL::LONG),
            (&segment_path, LENGTH_LEVEL::SHORT),
            (&segment_git, LENGTH_LEVEL::LONG),
        ],
        vec![
            (&segment_ssh, LENGTH_LEVEL::MEDIUM),
            (&segment_userhostname, LENGTH_LEVEL::LONG),
            (&segment_path, LENGTH_LEVEL::SHORT),
            (&segment_git, LENGTH_LEVEL::MEDIUM),
        ],
        vec![
            (&segment_ssh, LENGTH_LEVEL::MEDIUM),
            (&segment_userhostname, LENGTH_LEVEL::MEDIUM),
            (&segment_path, LENGTH_LEVEL::SHORT),
            (&segment_git, LENGTH_LEVEL::MEDIUM),
        ],
        vec![
            (&segment_ssh, LENGTH_LEVEL::SHORT),
            (&segment_userhostname, LENGTH_LEVEL::SHORT),
            (&segment_path, LENGTH_LEVEL::SHORT),
            (&segment_git, LENGTH_LEVEL::MEDIUM),
        ],
        vec![
            (&segment_ssh, LENGTH_LEVEL::SHORT),
            (&segment_userhostname, LENGTH_LEVEL::SHORT),
            (&segment_path, LENGTH_LEVEL::SHORT),
            (&segment_git, LENGTH_LEVEL::MEDIUM),
        ],
    ];

    // output
    for profile in profiles {
        let sum = (&profile)
            .iter()
            .filter(|(seg, level)| (*seg).is_enabled())
            .map(|(seg, level)| (**seg).get_size()[*level as usize] + 1)
            .sum();

        if width >= sum {
            let mut string = String::new();
            string.reserve(1024);

            for (i, &(seg, level)) in profile
                .iter()
                .filter(|(seg, level)| (*seg).is_enabled())
                .enumerate()
            {
                string.push_str(background((*seg).get_bg()).as_str());
                if i != 0 {
                    string.push(SYMBOL_RIGHT);
                }
                string.push_str(forground((*seg).get_fg()).as_str());
                string.push_str((*seg).construct(level, BuildMode::CONSTRUCT).data.as_str());
                string.push_str(forground((*seg).get_bg()).as_str());
            }

            string.push_str(resetbackground().as_str());
            string.push(SYMBOL_RIGHT);
            println!("{}", string);
            break;
        }
    }

    print!(
        "{} ",
        prompt
            .construct(LENGTH_LEVEL::LONG, BuildMode::CONSTRUCT)
            .data
    );
    return Ok(());
}
