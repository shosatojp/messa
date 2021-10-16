use crate::builder::*;
use crate::util::colors::*;
use crate::util::symbols::*;
use crate::util::*;
use crate::{
    builder::PromptStringBuilder,
    util::{self, PromptSegment},
};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader, path::Path};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct KubeConfig {
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
struct KubeUserConfig {
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct KubeClusterConfig {
    pub name: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct KubeContextConfig {
    pub name: String,
    pub context: KubeContext,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct KubeContext {
    pub cluster: String,
    pub namespace: String,
    pub user: String,
}

#[allow(non_snake_case)]
struct Kube {
    fg: String,
    bg: String,
    pub size: [u32; 3],
    pub config: KubeConfig,
}

impl Kube {
    pub fn new(
        fg: &str,
        bg: &str,
        home: &str,
        pwd: &str,
    ) -> Result<Kube, Box<dyn std::error::Error>> {
        let mut kube = Kube {
            fg: fg.to_string(),
            bg: bg.to_string(),
            size: [0, 0, 0],
            config: Kube::load_config("~/.kube/config")?,
        };

        kube.size[2] = kube
            .construct(LENGTH_LEVEL::LONG, BuildMode::ESTIMATE)
            .count as u32;
        kube.size[1] = kube
            .construct(LENGTH_LEVEL::MEDIUM, BuildMode::ESTIMATE)
            .count as u32;
        kube.size[0] = kube
            .construct(LENGTH_LEVEL::SHORT, BuildMode::ESTIMATE)
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
        return self.config.current_context.to_string();
    }

    fn get_namespace(&self) -> Option<&str> {
        let context_name = self.config.current_context.as_str();
        let context = match self.config.contexts.iter().find(|x| x.name == context_name) {
            Some(context_config) => context_config,
            None => return None,
        };
        return Some(context.context.namespace.as_str());
    }
}

impl PromptSegment for Kube {
    fn construct(
        &self,
        level: util::LENGTH_LEVEL,
        mode: crate::builder::BuildMode,
    ) -> crate::builder::PromptStringBuilder {
        let mut builder = PromptStringBuilder::new(mode);
        builder.push(' ');
        builder.push_string(&self.get_context());

        if level == util::LENGTH_LEVEL::LONG {
            match self.get_namespace() {
                Some(ns) => {
                    builder.push('/');
                    builder.push_string(&ns.to_owned());
                }
                None => (),
            }
        }

        return builder;
    }

    fn get_size(&self) -> &[u32; 3] {
        return &self.size;
    }
    fn get_fg(&self) -> &str {
        return &self.fg;
    }
    fn get_bg(&self) -> &str {
        return &self.bg;
    }
    fn is_enabled(&self) -> bool {
        return true;
    }
}

#[test]
fn test_load_config() -> Result<(), Box<dyn std::error::Error>> {
    let path = util::expand_user("~/.kube/config")?;
    let config = Kube::load_config(&path)?;
    println!("{:?}", &config.current_context);
    let context_name = config.current_context;
    let context = config
        .contexts
        .iter()
        .find(|x| x.name == context_name)
        .ok_or("context not found")?;
    println!("{:?}", context.context.namespace);
    Ok(())
}