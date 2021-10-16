use serde::de::Unexpected;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::ops::Deref;
use std::rc::Rc;
use std::{collections::HashMap, fs::File, io::BufReader, iter::Map, process::exit};

use crate::segments::kube::{Kube, KubeConfig};
use crate::segments::{git::Git, path::Path, ssh::Ssh, time::Time, userhost::UserHostname};
use crate::util::colors;
use crate::util::{
    colors::{background, forground},
    Location, PromptSegment, LENGTH_LEVEL,
};

type SegmentsMap = HashMap<String, Rc<Box<dyn PromptSegment>>>;

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
    // #[serde(deserialize_with = "location_deserialize")]
    // pub location: Location,
    pub location: Option<String>,
}

pub struct SegmentConfig {
    pub segment: Rc<Box<dyn PromptSegment>>,
    pub size: LENGTH_LEVEL,
    pub location: Location,
}

// fn location_deserialize<'de, D>(deserializer: D) -> Result<Location, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s = String::deserialize(deserializer)?;
//     match s.as_str() {
//         "left" => Ok(Location::LEFT),
//         "right" => Ok(Location::RIGHT),
//         _ => Err(de::Error::invalid_value(Unexpected::Str(&s), &"")),
//     }
// }

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct RawProfileConfig {
    segments: Vec<RawSegmentConfig>,
}

pub struct ProfileConfig {
    pub segments: Vec<SegmentConfig>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
pub struct RawConfig {
    pub config: HashMap<String, RawTypeConfig>,
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
    ) -> Result<ConfigLoader, Box<dyn std::error::Error>> {
        let config = ConfigLoader::load_config(path)?;
        let segments = ConfigLoader::build_segments(&config.config, pwd, home, user, hostname)?;
        Ok(ConfigLoader {
            path: path.to_string(),
            config,
            segments,
        })
    }
    fn load_config(path: &str) -> Result<RawConfig, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(&file);
        let config: RawConfig = serde_yaml::from_reader(reader)?;
        Ok(config)
    }

    pub fn build_segments(
        type_configs: &HashMap<String, RawTypeConfig>,
        pwd: &str,
        home: &str,
        user: &str,
        hostname: &str,
    ) -> Result<SegmentsMap, Box<dyn std::error::Error>> {
        let mut hashmap: SegmentsMap = HashMap::new();
        for (type_, type_config) in type_configs {
            let fg = colors::from_humanreadable(&type_config.fg);
            let bg = colors::from_humanreadable(&type_config.bg);
            let segment: Box<dyn PromptSegment> = match type_.as_str() {
                "ssh" => Box::new(Ssh::new(&fg, &bg)),
                "userhost" => Box::new(UserHostname::new(&fg, &bg, user, hostname)),
                "path" => Box::new(Path::new(&fg, &bg, home, pwd)),
                "git" => Box::new(Git::new(&fg, &bg, &pwd)),
                "time" => Box::new(Time::new(&fg, &bg)),
                "kube" => Box::new(Kube::new(&fg, &bg)?),
                _ => {
                    eprintln!("Unsupported segment type");
                    exit(1);
                }
            };

            hashmap.insert(type_.to_string(), Rc::new(segment));
        }

        Ok(hashmap)
    }

    pub fn build_profiles(&self) -> Result<Vec<ProfileConfig>, Box<dyn std::error::Error>> {
        // let segments = ConfigLoader::build_segments(type_configs, pwd, home, user, hostname)
        let mut profiles: Vec<ProfileConfig> = vec![];
        for raw_profile in &self.config.profiles {
            let segments: Vec<SegmentConfig> = raw_profile
                .segments
                .iter()
                .map(|e| SegmentConfig {
                    segment: Rc::clone(
                        self.segments
                            .get(&e.type_)
                            .expect(&format!("key not found: {}", e.type_)),
                    ),
                    location: Location::LEFT, // TODO
                    size: LENGTH_LEVEL::LONG, // TODO
                })
                .collect();
            let profile = ProfileConfig { segments };
            profiles.push(profile);
        }
        Ok(profiles)
    }
}