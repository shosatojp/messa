use crate::util::colors::color_code;

pub type GenericShell = Box<dyn Shell>;

pub trait Shell {
    fn set_fg(&self, config: &str) -> String;
    fn set_bg(&self, config: &str) -> String;
    fn resetbackground(&self) -> String;
    fn resetcolor(&self) -> String;
}

pub struct Bash {}

impl Bash {
    pub fn new() -> Bash {
        Bash {}
    }
}

impl Shell for Bash {
    fn set_fg(&self, config: &str) -> String {
        format!("\\[\x1b[38;5;{}m\\]", color_code(config))
    }

    fn set_bg(&self, config: &str) -> String {
        format!("\\[\x1b[48;5;{}m\\]", color_code(config))
    }

    fn resetbackground(&self) -> String {
        String::from("\\[\x1b[49;24m\\]")
    }

    fn resetcolor(&self) -> String {
        String::from("\\[\x1b[0m\\]")
    }
}

pub struct Zsh {}

impl Zsh {
    pub fn new() -> Zsh {
        Zsh {}
    }
}

impl Shell for Zsh {
    fn set_fg(&self, config: &str) -> String {
        format!("%{{\x1b[38;5;{}m%}}", color_code(config))
    }

    fn set_bg(&self, config: &str) -> String {
        format!("%{{\x1b[48;5;{}m%}}", color_code(config))
    }

    fn resetbackground(&self) -> String {
        format!("%{{\x1b[49;24m%}}")
    }

    fn resetcolor(&self) -> String {
        format!("%{{\x1b[0m%}}")
    }
}

pub struct Fish {}

impl Fish {
    pub fn new() -> Fish {
        Fish {}
    }
}

impl Shell for Fish {
    fn set_fg(&self, config: &str) -> String {
        format!("\x1b[38;5;{}m", color_code(config))
    }

    fn set_bg(&self, config: &str) -> String {
        format!("\x1b[48;5;{}m", color_code(config))
    }

    fn resetbackground(&self) -> String {
        format!("\x1b[49;24m")
    }

    fn resetcolor(&self) -> String {
        format!("\x1b[0m")
    }
}
