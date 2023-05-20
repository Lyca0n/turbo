use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BuilderConfig {
    pub kinds: Vec<Kind>,
    pub templates: Vec<Template>,
}

impl BuilderConfig {
    pub fn from_file(path: String) -> Self {
        let f = File::open(path).expect("Unable to open configuration file");
        serde_yaml::from_reader(f).expect("failed to parse configuration file for project builder")
    }

    pub fn has_kind(&self, name: String) -> bool {
        self.kinds.iter().cloned().find(|kind| kind.name == name).is_some()
    }

    pub fn kind_names(&self) -> Vec<String> {
       let mut x = Vec::new();
       for k in self.kinds.iter() {
            x.push(k.name.clone());
       }
       return x;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kind {
    pub name: String,
    pub description: String,
    pub kinds: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    kind: String,
    tool: String,
    location: String,
    inputs: Vec<Input>,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Input {
    prompt: String,
    key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    prompt: String,
    key: String,
}
