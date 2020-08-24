use git2::{Branch, Repository, Status, StatusOptions};

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

pub const SYMBOL_RIGHT: char = '\u{e0b0}';
pub const SYMBOL_RIGHT_ALT: char = '\u{e0b1}';
pub const SYMBOL_GIT_UNPUSHED: char = '↑';
pub const SYMBOL_GIT_BRANCH: char = '\u{e0a0}';
pub const SYMBOL_GIT_CHANGED: char = '\x2a';
pub const SYMBOL_GIT_STAGED: char = '\x2b';

fn forground(color: &str) -> String {
    return format!("\x1b[38;{}m", color);
}

fn background(color: &str) -> String {
    return format!("\x1b[48;{}m", color);
}

fn resetbackground() -> String {
    return String::from("\x1b[49;24m");
}

fn resetcolor() -> String {
    return String::from("\x1b[0m");
}

fn get_branch_name(repo: &Repository) -> String {
    let branch = Branch::wrap(repo.head().unwrap());
    return match branch.name() {
        Ok(name) => format!(" {}", name.unwrap()),
        Err(_) => String::from(""),
    };
}

fn main() {
    let repo = Repository::open("/home/sho/repos/powerline-shell");
    let mut prompt = String::new();

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
    prompt.push_str(" /home/sho ");
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


    
}
