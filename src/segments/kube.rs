use crate::builder::*;
use crate::util::colors::RawAppearance;
use crate::util::*;
use crate::{
    builder::PromptStringBuilder,
    util::{self, PromptSegment},
};
use serde::Deserialize;
use serde::Serialize;
use std::{fs::File, io::BufReader};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct KubeConfig {
    pub apiVersion: String,
    pub clusters: Vec<KubeClusterConfig>,
    pub contexts: Vec<KubeContextConfig>,
    #[serde(rename = "current-context")]
    pub current_context: String,
    pub kind: String,
    pub users: Vec<KubeUserConfig>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct KubeUserConfig {
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct KubeClusterConfig {
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct KubeContextConfig {
    pub name: String,
    pub context: KubeContext,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct KubeContext {
    pub cluster: String,
    pub namespace: String,
    pub user: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RawKubeConfig {
    pub appearance: RawAppearance,
    pub icon: Option<String>,
}

#[allow(non_snake_case)]
pub struct Kube {
    config: RawKubeConfig,
    pub size: [u32; 3],
    pub kube_config: KubeConfig,
}

impl Kube {
    pub fn new(
        config: &RawKubeConfig,
        kube_config_path: &str,
    ) -> Result<Kube, Box<dyn std::error::Error>> {
        let mut kube = Kube {
            config: config.clone(),
            size: [0, 0, 0],
            kube_config: Kube::load_config(kube_config_path)?,
        };

        kube.size[2] = kube.construct(LengthLevel::LONG, BuildMode::ESTIMATE).count as u32;
        kube.size[1] = kube
            .construct(LengthLevel::MEDIUM, BuildMode::ESTIMATE)
            .count as u32;
        kube.size[0] = kube
            .construct(LengthLevel::SHORT, BuildMode::ESTIMATE)
            .count as u32;

        return Ok(kube);
    }

    fn load_config(path: &str) -> Result<KubeConfig, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config: KubeConfig = serde_yaml::from_reader(reader)?;
        Ok(config)
    }

    fn get_context(&self) -> String {
        return self.kube_config.current_context.to_string();
    }

    fn get_namespace(&self) -> Option<&str> {
        let context_name = self.kube_config.current_context.as_str();
        let context = match self
            .kube_config
            .contexts
            .iter()
            .find(|x| x.name == context_name)
        {
            Some(context_config) => context_config,
            None => return None,
        };
        return Some(context.context.namespace.as_str());
    }
}

impl PromptSegment for Kube {
    fn construct(
        &self,
        level: util::LengthLevel,
        mode: crate::builder::BuildMode,
    ) -> crate::builder::PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);
        builder.push(' ');
        if let Some(icon) = &self.config.icon {
            builder.push_string(icon);
        }

        match level {
            util::LengthLevel::LONG => {
                builder.push_string(&self.get_context());
                match self.get_namespace() {
                    Some(ns) => {
                        builder.push('/');
                        builder.push_string(&ns.to_owned());
                    }
                    None => (),
                }
            }
            util::LengthLevel::MEDIUM | util::LengthLevel::SHORT => match self.get_namespace() {
                Some(ns) => {
                    builder.push_string(&ns.to_owned());
                }
                None => (),
            },
        }
        builder.push(' ');

        return builder;
    }

    fn get_size(&self) -> &[u32; 3] {
        return &self.size;
    }
    fn get_fg(&self) -> String {
        return self.config.appearance.get_fg().to_string();
    }
    fn get_bg(&self) -> String {
        return self.config.appearance.get_bg().to_string();
    }
    fn is_enabled(&self) -> bool {
        return true;
    }
}

#[test]
fn test_load_config() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = std::env::var("HOME").unwrap();
    let path = util::expand_user(&home_dir, "~/.kube/config");
    let config = Kube::load_config(&path)?;
    let context_name = config.current_context;
    let context = config
        .contexts
        .iter()
        .find(|x| x.name == context_name)
        .ok_or("context not found")?;
    println!("namespace: {}", &context.context.namespace);
    Ok(())
}
