mod args;
mod util;
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

mod builder;
use clap::ArgMatches;
mod out;
use out::*;
mod config;

fn main() -> Result<(), String> {
    let matches: ArgMatches = get_arg_matches();

    // arguments
    let pwd = matches
        .value_of("pwd")
        .and_then(|e| Some(e.to_string()))
        .unwrap_or({
            let current_dir = std::env::current_dir().or(Err("failed to get current dir"))?;
            current_dir.to_str().unwrap().to_string()
        });
    let home = matches
        .value_of("home")
        .and_then(|e| Some(e.to_string()))
        .unwrap_or(std::env::var("HOME").or(Err("failed to get current dir"))?);
    let width: u32 = matches
        .value_of("width")
        .unwrap()
        .parse()
        .or(Err("failed to parse width"))?;
    let prev_error: u8 = matches
        .value_of("error")
        .unwrap()
        .parse()
        .or(Err("failed to parse error"))?;
    let user = matches.value_of("user").unwrap().to_string();
    let hostname = matches.value_of("host").unwrap().to_string();
    let kube_config_path = util::expand_user(&home, matches.value_of("kubeconfig").unwrap());
    let config_path = util::expand_user(&home, matches.value_of("config").unwrap());

    let loader = config::ConfigLoader::new(
        &config_path,
        &pwd,
        &home,
        &user,
        &hostname,
        &kube_config_path,
        prev_error,
    )
    .or_else(|e| Err(e.to_string()))?;
    let profiles = loader.build_profiles().or_else(|e| Err(e.to_string()))?;

    out(width, &profiles, loader.get_prompt());

    Ok(())
}
