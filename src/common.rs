use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::util::git_clone;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template {
    pub kind: String,
    pub tool: String,
    pub location: String,
    pub extensions: String,
    #[serde(default)]
    pub inputs: Vec<Input>,
    #[serde(default)]
    pub choices: Vec<Choice>,
}

/**
 * Struct meant to describe a project template to process
 */
impl Template {
    pub fn fill_inputs(&mut self, callback: fn(&Input) -> String) {
        for input in self.inputs.iter_mut() {
            let res = callback(&input);
            input.value = Some(res);
        }
    }

    pub fn fill_choices(&mut self, callback: fn(&Choice) -> bool) {
        for choice in self.choices.iter_mut() {
            let res = callback(&choice);
            choice.value = Some(res);
        }
    }

    pub fn clone_source(&self, dest: &Path){
        let cloned = git_clone(dest, &self.location);
        
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Input {
    pub prompt: String,
    key: String,
    value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Choice {
    pub prompt: String,
    key: String,
    value: Option<bool>,
}
