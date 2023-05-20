use serde::{Deserialize, Serialize};
use serde_yaml;
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct BuilderConfig {
    pub types: Vec<Type>,
    pub templates: Vec<Template>,
}

impl BuilderConfig {
    pub fn from_file(path: String) -> Self {
        let f = File::open(path).expect("Unable to open configuration file");
        serde_yaml::from_reader(f).expect("failed to parse configuration file for project builder")
    }

    pub fn type_names(&self) -> Vec<&str> {
        self.types.iter().map(|k| k.name.as_str()).collect()
    }

    pub fn get_type_by_name(&self, name: &str) -> Option<Type> {
        self.types.iter().find(|t| t.name.as_str() == name).cloned()
    }

    pub fn get_template_by_kind(&self, name: &str) -> Option<Template> {
        self.templates.iter().find(|t| t.kind == name).cloned()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Type {
    pub name: String,
    pub description: String,
    pub kinds: Vec<String>,
}

impl Type {
    pub fn menu_print(&self) -> String {
        format!("{} {}", self.name, self.description)
    }

    pub fn kind_names(&self) -> Vec<&str> {
        self.kinds.iter().map(|k| k.as_str()).collect()
    }
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
