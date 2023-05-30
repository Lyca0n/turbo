use std::{fs::OpenOptions, io::Write, path::Path};

use tera::{Context, Tera};

use crate::error::Error;

pub struct Renderer {
    extensions: String,
    dir: String,
    templater: Option<Tera>,
    context: Context,
}
impl Renderer {
    pub fn new(extensions: String, dir: String) -> Self {
        Self {
            extensions,
            dir,
            templater: None,
            context: Context::new(),
        }
    }

    pub fn init_template(&mut self) -> Result<(), Error> {
        match Tera::new(self.tera_mask().as_str()) {
            Ok(t) => {
                self.templater = Some(t);
                Ok(())
            }
            Err(e) => Err(Error::Generic(e.to_string())),
        }
    }

    pub fn add_to_context(&mut self, key: &str, value: &str) {
        self.context.insert(key, value);
    }

    pub fn render_all_in_place(&self) -> Result<(), Error> {
        if let Some(tera) = self.templater.as_ref() {
            for name in tera.get_template_names() {
                println!("file {}", name);
                let file_path = Path::new(self.dir.as_str()).join(name);
                let replacer = self
                    .templater
                    .as_ref()
                    .unwrap()
                    .render(name, &self.context)
                    .unwrap();
                let mut f = OpenOptions::new()
                    .write(true)
                    .open(file_path.as_path())
                    .unwrap();
                let _ = f.write_all(replacer.as_bytes());
                let _ = f.flush();
            }
            Ok(())
        } else {
            Err(Error::Generic("templater not initialized".to_owned()))
        }
    }

    fn tera_mask(&self) -> String {
        format!("{}**/*.{{{}}}", &self.dir, &self.extensions)
    }
}
