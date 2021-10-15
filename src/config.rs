use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader, iter::Map, process::exit};

use crate::segments::{
    git::Git, path::Path, ssh::Ssh, time::Time, userhost::UserHostname,
};
use crate::util::PromptSegment;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct TypeConfig {
    pub fg: String,
    pub bg: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct SegmentConfig {
    #[serde(rename = "type")]
    pub type_: String,
    pub size: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct ProfileConfig {
    segments: Vec<SegmentConfig>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct Config {
    pub config: HashMap<String, TypeConfig>,
    pub profiles: Vec<ProfileConfig>,
}

struct ConfigLoader {
    pub path: String,
    pub config: Config,
    pub segments: HashMap<String, Box<dyn PromptSegment>>,
}

impl ConfigLoader {
    pub fn new(path: &str) -> Result<ConfigLoader, Box<dyn std::error::Error>> {
        return Ok(ConfigLoader {
            path: path.to_string(),
            config: ConfigLoader::load_config(path)?,
            segments: HashMap::new(),
        });
    }

    fn load_config(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(&file);
        let config: Config = serde_yaml::from_reader(reader)?;
        Ok(config)
    }

    pub fn build_segments(&mut self, pwd: &str, home: &str, user: &str, hostname: &str) {
        for (type_, type_config) in &self.config.config {
            let segment: Box<dyn PromptSegment> = match type_.as_str() {
                "ssh" => Box::new(Ssh::new(&type_config.fg, &type_config.bg)),
                "userhost" => Box::new(UserHostname::new(
                    &type_config.fg,
                    &type_config.bg,
                    user,
                    hostname,
                )),
                "path" => Box::new(Path::new(&type_config.fg, &type_config.bg, home, pwd)),
                "git" => Box::new(Git::new(&type_config.fg, &type_config.bg, &pwd)),
                "time" => Box::new(Time::new(&type_config.fg, &type_config.bg)),
                _ => {
                    eprintln!("Unsupported segment type");
                    exit(1);
                }
            };

            self.segments.insert(type_.to_string(), segment);
        }
    }
}
