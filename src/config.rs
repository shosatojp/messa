use serde::Deserialize;
use std::rc::Rc;
use std::{collections::HashMap, fs::File, io::BufReader, process::exit};

use crate::segments::kube::Kube;
use crate::segments::{git::Git, path::Path, ssh::Ssh, time::Time, userhost::UserHostname};
use crate::util;
use crate::util::colors;
use crate::util::{LengthLevel, Location, PromptSegment};

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
        kube_config_path: &str,
    ) -> Result<ConfigLoader, Box<dyn std::error::Error>> {
        let config = ConfigLoader::load_config(path)?;
        let segments = ConfigLoader::build_segments(
            &config.config,
            pwd,
            home,
            user,
            hostname,
            kube_config_path,
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
        type_configs: &HashMap<String, RawTypeConfig>,
        pwd: &str,
        home: &str,
        user: &str,
        hostname: &str,
        kube_config_path: &str,
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
                "kube" => Box::new(Kube::new(kube_config_path, &fg, &bg)?),
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
        let mut profiles: Vec<ProfileConfig> = vec![];
        for raw_profile in &self.config.profiles {
            let segments: Vec<SegmentConfig> = raw_profile
                .segments
                .iter()
                .map(|e| SegmentConfig {
                    segment: Rc::clone(self.segments.get(&e.type_).unwrap_or_else(|| {
                        eprintln!("Segment not found for key `{}`. Please setup `{}` segment on your config.", e.type_, e.type_);
                        exit(1);
                    })),
                    location: util::load_location(&e.location.as_ref().unwrap_or(&"left".to_string())),
                    size: util::load_lengthlevel(&e.size),
                })
                .collect();
            let profile = ProfileConfig { segments };
            profiles.push(profile);
        }
        Ok(profiles)
    }
}
