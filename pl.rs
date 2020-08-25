#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use git2::{Branch, Repository};
mod lib;
use lib::colors::*;
use lib::symbols::*;
use lib::*;
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
mod prompt;
use clap::ArgMatches;
use prompt::*;

fn main() -> Result<(), &'static str> {
    let matches: ArgMatches = get_arg_matches();

    let pwd = matches.value_of("pwd").ok_or("")?;
    let home = matches.value_of("home").unwrap();

    let width: u32 = matches.value_of("width").unwrap().parse().unwrap();
    let prev_error: i8 = matches
        .value_of("error")
        .unwrap()
        .parse()
        .or(Err("error must be int"))?;

    // def colors
    let fg = WHITE;
    let bg_user_hostname = INDIGO;
    let bg_path = TEAL;
    let bg_git = DEEP_ORANGE;
    let bg_prompt = if prev_error > 0 { PINK } else { CYAN };

    // partial prompt builders
    let userhostname = UserHostname::new(fg, bg_user_hostname);
    let path = Path::new(fg, bg_path, home, pwd);
    let git = Git::new(fg, bg_git, pwd);
    // let partials: [Box<dyn PartialPrompt>; 3] =
    //     [Box::new(userhostname), Box::new(path), Box::new(git)];
    let prompt = Prompt::new(fg, bg_prompt, prev_error);

    let profiles: Vec<[LENGTH_LEVEL; 3]> = vec![
        [LENGTH_LEVEL::LONG, LENGTH_LEVEL::LONG, LENGTH_LEVEL::LONG],
        [LENGTH_LEVEL::LONG, LENGTH_LEVEL::MEDIUM, LENGTH_LEVEL::LONG],
        [LENGTH_LEVEL::LONG, LENGTH_LEVEL::SHORT, LENGTH_LEVEL::LONG],
        [
            LENGTH_LEVEL::LONG,
            LENGTH_LEVEL::SHORT,
            LENGTH_LEVEL::MEDIUM,
        ],
        [
            LENGTH_LEVEL::MEDIUM,
            LENGTH_LEVEL::SHORT,
            LENGTH_LEVEL::MEDIUM,
        ],
        [
            LENGTH_LEVEL::SHORT,
            LENGTH_LEVEL::SHORT,
            LENGTH_LEVEL::MEDIUM,
        ],
        [
            LENGTH_LEVEL::SHORT,
            LENGTH_LEVEL::SHORT,
            LENGTH_LEVEL::MEDIUM,
        ],
    ];

    // for profile in profiles {
    //     let mut sum = 0;
    //     for (i, level) in profile.iter().enumerate() {
    //         sum += partials[i].get_size()[*level as usize] + 1;
    //     }

    //     if width >= sum {
    //         let mut string = String::new();
    //         string.reserve(1024);

    //         for (i, part) in partials.iter().enumerate() {
    //             string.push_str(background((*part).get_bg()).as_str());
    //             if (i != 0) {
    //                 string.push(SYMBOL_RIGHT);
    //             }
    //             string.push_str(forground((*part).get_fg()).as_str());

    //             string.push_str(
    //                 part.construct(profile[i], BuildMode::CONSTRUCT)
    //                     .data
    //                     .as_str(),
    //             );

    //             string.push_str(forground((*part).get_bg()).as_str());
    //         }
    //     }
    // }

    for profile in profiles {
        if width
            >= userhostname.size[profile[0] as usize]
                + path.size[profile[1] as usize]
                + git.size[profile[2] as usize]
        {
            let mut string = String::new();
            string.reserve(1024);
            string.push_str(
                userhostname
                    .construct(profile[0], BuildMode::CONSTRUCT)
                    .data
                    .as_str(),
            );
            string.push_str(
                path.construct(profile[1], BuildMode::CONSTRUCT)
                    .data
                    .as_str(),
            );
            string.push_str(
                git.construct(profile[2], BuildMode::CONSTRUCT)
                    .data
                    .as_str(),
            );
            println!("{}", string);
            break;
        }
    }

    println!(
        "{}",
        prompt
            .construct(LENGTH_LEVEL::LONG, BuildMode::CONSTRUCT)
            .data
    );
    return Ok(());
}
