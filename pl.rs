#![allow(dead_code)]
#![allow(non_camel_case_types)]

use git2::{Branch, Repository};
mod lib;
use lib::colors::*;
use lib::symbols::*;
use lib::*;
mod args;
use args::*;

fn main() -> Result<(), &'static str> {
    let matches = get_arg_matches();
    let pwd = matches.value_of("pwd").unwrap();
    let home = matches.value_of("home").unwrap();
    let prev_error: i8 = matches
        .value_of("error")
        .unwrap()
        .parse()
        .or(Err("error must be int"))?;

    let repo = Repository::open(pwd);
    let mut prompt = String::new();
    prompt.reserve(1024);

    prompt.push_str(forground(WHITE).as_str());
    prompt.push_str(background(INDIGO).as_str());
    prompt.push(' ');
    prompt.push_str(whoami::username().as_str());
    prompt.push('@');
    prompt.push_str(whoami::hostname().as_str());
    prompt.push(' ');
    prompt.push_str(forground(INDIGO).as_str());
    prompt.push_str(background(TEAL).as_str());
    prompt.push(SYMBOL_RIGHT);
    prompt.push_str(forground(WHITE).as_str());
    prompt.push(' ');
    prompt.push_str(build_path_str(home, pwd, PATH_LENGTH::LONG).as_str());
    prompt.push(' ');
    if repo.is_ok() {
        prompt.push_str(forground(TEAL).as_str());
        prompt.push_str(background(DEEP_ORANGE).as_str());
        prompt.push(SYMBOL_RIGHT);
        prompt.push_str(forground(WHITE).as_str());
        prompt.push(' ');
        prompt.push(SYMBOL_GIT_BRANCH);

        // git
        let re = repo.unwrap();
        let branch = Branch::wrap(re.head().unwrap());

        // branch name
        branch.name().ok().and_then(|opt_name| {
            opt_name.and_then(|name| Some(prompt.push_str(format!(" {}", name).as_str())))
        });

        // changed & staged
        let (changed, staged) = count_git_status(&re);
        if changed > 0 {
            prompt.push(SYMBOL_GIT_CHANGED);
        }
        if staged > 0 {
            prompt.push(SYMBOL_GIT_STAGED);
        }

        // unpushed
        count_unpushed(&re, &branch).ok().and_then(|unpushed| {
            Some(prompt.push_str(format!(" {}{}", SYMBOL_GIT_UNPUSHED, unpushed).as_str()))
        });

        prompt.push(' ');
        prompt.push_str(resetbackground().as_str());
        prompt.push_str(forground(DEEP_ORANGE).as_str());
        prompt.push(SYMBOL_RIGHT);
    } else {
        prompt.push_str(forground(TEAL).as_str());
        prompt.push_str(resetbackground().as_str());
        prompt.push(SYMBOL_RIGHT);
    }
    prompt.push_str(resetcolor().as_str());
    prompt.push('\n');

    prompt.push_str(forground(WHITE).as_str());
    if prev_error > 0 {
        prompt.push_str(background(PINK).as_str());
        prompt.push_str("🤗 ");
        prompt.push_str(format!("{}", prev_error).as_str());
        prompt.push_str(" $ ");
        prompt.push_str(resetcolor().as_str());
        prompt.push_str(forground(PINK).as_str());
    } else {
        prompt.push_str(background(CYAN).as_str());
        prompt.push_str("🤗 ");
        prompt.push_str(" $ ");
        prompt.push_str(resetcolor().as_str());
        prompt.push_str(forground(CYAN).as_str());
    }
    prompt.push_str(resetbackground().as_str());
    prompt.push(SYMBOL_RIGHT);
    prompt.push_str(resetcolor().as_str());

    println!("{}", prompt);

    return Ok(());
}
