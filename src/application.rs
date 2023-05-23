use crate::common::{Choice, Template};

pub struct Application {
    pub app_type: String,
    pub template: Template,
    pub customizations: Vec<Choice>,
}
