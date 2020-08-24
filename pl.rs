use git2::Repository;
mod lib;
use lib::colors::*;
use lib::symbols::*;
use lib::*;
mod args;
use args::*;

fn main() {
    let matches = get_arg_matches();
    let pwd = matches.value_of("pwd").unwrap();
    let home = matches.value_of("home").unwrap();
    let prev_error: i8 = matches.value_of("error").unwrap().parse().unwrap();

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
        //git
        let staged_mask = 0b11111;
        let changed_mask = 0b11111 << 7;

        let mut changed = 0;
        let mut staged = 0;
        let re = repo.unwrap();
        re.statuses(Option::None)
            .unwrap()
            .iter()
            .for_each(|status| {
                let bits = &status.status().bits();
                changed += std::cmp::min(bits & changed_mask, 1);
                staged += std::cmp::min(bits & staged_mask, 1);
                return ();
            });
        prompt.push_str(get_branch_name(&re).as_str());
        if changed > 0 {
            prompt.push(SYMBOL_GIT_CHANGED);
        }
        if staged > 0 {
            prompt.push(SYMBOL_GIT_STAGED);
        }
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
}
