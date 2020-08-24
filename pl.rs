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

    // def colors
    let fg = WHITE;
    let bg_user_hostname = INDIGO;
    let bg_path = TEAL;
    let bg_git = DEEP_ORANGE;
    let bg_prompt = if prev_error > 0 { PINK } else { CYAN };

    // user@hostname
    prompt.push_str(background(bg_user_hostname).as_str());
    prompt.push_str(forground(fg).as_str());
    prompt.push(' ');
    prompt.push_str(whoami::username().as_str());
    prompt.push('@');
    prompt.push_str(whoami::hostname().as_str());
    prompt.push(' ');
    prompt.push_str(forground(bg_user_hostname).as_str());
    prompt.push_str(background(bg_path).as_str());
    prompt.push(SYMBOL_RIGHT);

    // path
    prompt.push_str(forground(fg).as_str());
    prompt.push(' ');
    prompt.push_str(build_path_str(home, pwd, PATH_LENGTH::LONG).as_str());
    prompt.push(' ');

    // git
    match repo {
        Ok(repo) => {
            prompt.push_str(forground(bg_path).as_str());
            prompt.push_str(background(bg_git).as_str());
            prompt.push(SYMBOL_RIGHT);
            prompt.push_str(forground(fg).as_str());
            prompt.push(' ');
            prompt.push(SYMBOL_GIT_BRANCH);

            // git
            let branch = Branch::wrap(repo.head().unwrap());

            // branch name
            branch.name().ok().and_then(|opt_name| {
                opt_name.and_then(|name| Some(prompt.push_str(format!(" {}", name).as_str())))
            });

            // changed & staged
            let (changed, staged) = count_git_status(&repo);
            if changed > 0 {
                prompt.push(SYMBOL_GIT_CHANGED);
            }
            if staged > 0 {
                prompt.push(SYMBOL_GIT_STAGED);
            }

            // unpushed
            count_unpushed(&repo, &branch).ok().and_then(|unpushed| {
                Some(prompt.push_str(format!(" {}{}", SYMBOL_GIT_UNPUSHED, unpushed).as_str()))
            });

            prompt.push(' ');
            prompt.push_str(resetbackground().as_str());
            prompt.push_str(forground(bg_git).as_str());
            prompt.push(SYMBOL_RIGHT);
        }
        Err(_) => {
            prompt.push_str(forground(bg_path).as_str());
            prompt.push_str(resetbackground().as_str());
            prompt.push(SYMBOL_RIGHT);
        }
    }
    prompt.push_str(resetcolor().as_str());
    prompt.push('\n');

    // prompt
    prompt.push_str(background(bg_prompt).as_str());
    prompt.push_str(forground(fg).as_str());
    prompt.push_str(
        format!(
            "🤗 {} $ ",
            if prev_error > 0 {
                prev_error.to_string()
            } else {
                "".to_string()
            }
        )
        .as_str(),
    );
    prompt.push_str(forground(bg_prompt).as_str());
    prompt.push_str(resetbackground().as_str());
    prompt.push(SYMBOL_RIGHT);
    prompt.push_str(resetcolor().as_str());

    println!("{}", prompt);

    return Ok(());
}
