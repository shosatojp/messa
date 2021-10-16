use serde::Deserialize;
use std::rc::Rc;
use std::{collections::HashMap, fs::File, io::BufReader, process::exit};

use crate::segments::git::RawGitConfig;
use crate::segments::kube::{Kube, RawKubeConfig};
use crate::segments::path::RawPathConfig;
use crate::segments::prompt::{Prompt, RawPromptConfig};
use crate::segments::ssh::RawSshConfig;
use crate::segments::time::RawTimeConfig;
use crate::segments::userhost::RawUserhostConfig;
use crate::segments::{git::Git, path::Path, ssh::Ssh, time::Time, userhost::UserHostname};
use crate::util;
use crate::util::{LengthLevel, Location, PromptSegment};

pub type Segment = Rc<Box<dyn PromptSegment>>;
pub type SegmentsMap = HashMap<String, Segment>;

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct RawTypeConfig {
    pub fg: String,
    pub bg: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct RawSegmentConfig {
    #[serde(rename = "type")]
    pub type_: String,
    pub size: String,
    pub location: Option<String>,
}

pub struct SegmentConfig {
    pub segment: Rc<Box<dyn PromptSegment>>,
    pub size: LengthLevel,
    pub location: Location,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct RawProfileConfig {
    segments: Vec<RawSegmentConfig>,
}

pub struct ProfileConfig {
    pub segments: Vec<SegmentConfig>,
}

#[derive(Deserialize, Debug)]
pub struct RawSegmentsConfig {
    pub ssh: Option<RawSshConfig>,
    pub git: Option<RawGitConfig>,
    pub userhost: Option<RawUserhostConfig>,
    pub time: Option<RawTimeConfig>,
    pub path: Option<RawPathConfig>,
    pub kube: Option<RawKubeConfig>,
    pub prompt: Option<RawPromptConfig>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct RawConfig {
    pub config: RawSegmentsConfig,
    pub profiles: Vec<RawProfileConfig>,
}

pub struct ConfigLoader {
    pub path: String,
    pub config: RawConfig,
    pub segments: SegmentsMap,
}

impl ConfigLoader {
    pub fn new(
        path: &str,
        pwd: &str,
        home: &str,
        user: &str,
        hostname: &str,
        kube_config_path: &str,
        prev_error: u8,
    ) -> Result<ConfigLoader, Box<dyn std::error::Error>> {
        let config = ConfigLoader::load_config(path)?;
        let segments = ConfigLoader::build_segments(
            &config.config,
            pwd,
            home,
            user,
            hostname,
            kube_config_path,
            prev_error,
        )?;
        Ok(ConfigLoader {
            path: path.to_string(),
            config,
            segments,
        })
    }
    fn load_config(path: &str) -> Result<RawConfig, Box<dyn std::error::Error>> {
        let file = File::open(path).unwrap_or_else(|_| {
            eprintln!("Unable to open config file: {}", &path);
            exit(1);
        });
        let reader = BufReader::new(&file);
        let config: RawConfig = serde_yaml::from_reader(reader)?;
        Ok(config)
    }

    pub fn build_segments(
        segments_config: &RawSegmentsConfig,
        pwd: &str,
        home: &str,
        user: &str,
        hostname: &str,
        kube_config_path: &str,
        prev_error: u8,
    ) -> Result<SegmentsMap, Box<dyn std::error::Error>> {
        let mut hashmap: SegmentsMap = HashMap::new();
        if let Some(ssh) = &segments_config.ssh {
            hashmap.insert(String::from("ssh"), Rc::new(Box::new(Ssh::new(ssh))));
        }
        if let Some(git) = &segments_config.git {
            hashmap.insert(String::from("git"), Rc::new(Box::new(Git::new(git, pwd))));
        }
        if let Some(time) = &segments_config.time {
            hashmap.insert(String::from("time"), Rc::new(Box::new(Time::new(time))));
        }
        if let Some(userhost) = &segments_config.userhost {
            hashmap.insert(
                String::from("userhost"),
                Rc::new(Box::new(UserHostname::new(userhost, user, hostname))),
            );
        }
        if let Some(path) = &segments_config.path {
            hashmap.insert(
                String::from("path"),
                Rc::new(Box::new(Path::new(path, home, pwd))),
            );
        }
        if let Some(kube) = &segments_config.kube {
            hashmap.insert(
                String::from("kube"),
                Rc::new(Box::new(Kube::new(kube, kube_config_path)?)),
            );
        }
        if let Some(prompt) = &segments_config.prompt {
            hashmap.insert(
                "prompt".to_string(),
                Rc::new(Box::new(Prompt::new(prompt, user, prev_error))),
            );
        }

        Ok(hashmap)
    }

    pub fn build_profiles(&self) -> Result<Vec<ProfileConfig>, Box<dyn std::error::Error>> {
        let mut profiles: Vec<ProfileConfig> = vec![];
        for raw_profile in &self.config.profiles {
            let mut segments: Vec<SegmentConfig> = vec![];
            for segment in &raw_profile.segments {
                let seg = match self.segments.get(&segment.type_) {
                    Some(e) => Rc::clone(e),
                    None => continue,
                };
                segments.push(SegmentConfig {
                    segment: seg,
                    location: util::load_location(
                        &segment.location.as_ref().unwrap_or(&"left".to_string()),
                    ),
                    size: util::load_lengthlevel(&segment.size),
                });
            }
            let profile = ProfileConfig { segments };
            profiles.push(profile);
        }
        Ok(profiles)
    }

    pub fn get_prompt(&self) -> &Segment {
        let segment = self.segments.get("prompt").unwrap_or_else(|| {
            eprintln!("`prompt` must be setup");
            exit(1);
        });
        segment
    }
}
