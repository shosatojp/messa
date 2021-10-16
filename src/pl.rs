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
mod segments {
    pub mod git;
    pub mod kube;
    pub mod path;
    pub mod prompt;
    pub mod ssh;
    pub mod time;
    pub mod userhost;
}

use segments::{git::*, kube::*, path::*, prompt::*, ssh::*, time::*, userhost::*};
mod builder;
use builder::*;
use clap::ArgMatches;
mod out;
use out::*;
mod config;

fn main() -> Result<(), String> {
    let matches: ArgMatches = get_arg_matches();

    // arguments
    let pwd = matches.value_of("pwd").unwrap().to_string();
    let home = matches.value_of("home").unwrap().to_string();

    let width: u32 = match matches.value_of("width").unwrap().parse() {
        Ok(width) => width,
        Err(_) => return,
    };
    let prev_error: u8 = match matches.value_of("error").unwrap().parse() {
        Ok(e) => e,
        Err(_) => return,
    };
    let user = matches.value_of("user").unwrap().to_string();
    let hostname = matches.value_of("host").unwrap().to_string();

    let config_path = "config.yaml";
    let loader = config::ConfigLoader::new(config_path, &pwd, &home, &user, &hostname)
        .or_else(|e| Err(e.to_string()))?;
    let profiles = loader.build_profiles().or_else(|e| Err(e.to_string()))?;

    let prompt = Prompt::new(
        &user,
        colors::from_humanreadable("red"),
        colors::from_humanreadable("white"),
        prev_error,
    );

    out(width, &profiles, &prompt);

    Ok(())
}
