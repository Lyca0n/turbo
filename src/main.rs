#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

use clap::Parser;
use inquire::Confirm;
use inquire::Select;
use inquire::Text;
use tera::Context;
use tera::Error;
use tera::Tera;
use turbo::cmd::{Cli, Commands};
use turbo::config::BuilderConfig;
use turbo::VERSION;

fn main() {
    check_prerequisites();
    let path = "./tree.yml";
    let config = BuilderConfig::from_file(path.to_string());
    let args = Cli::parse();
    match args.command {
        Commands::Version {} => {
            println!("turbo {VERSION}")
        }
        Commands::New { name } => {
            let project_path = Path::new(&name);
            println!("Select a type of project");
            let app_type_name = Select::new("Select project type", config.type_names())
                .prompt()
                .unwrap();
            let app_type = config.get_type_by_name(app_type_name).unwrap();
            let type_kind_name = Select::new(
                &format!("Select kind of {}", app_type_name),
                app_type.kind_names(),
            )
            .prompt()
            .unwrap();
            let mut template = config.get_template_by_kind(type_kind_name).unwrap();

            template.fill_inputs(|input| Text::new(input.prompt.as_str()).prompt().unwrap());

            template.fill_choices(|choice| {
                Confirm::new(choice.prompt.as_str())
                    .with_default(true)
                    .prompt()
                    .unwrap()
            });

            //prepare files
            let _ = fs::create_dir(project_path);
            let path = Path::new(project_path);
            let _ = template.clone_source(path);
            //try tera
            let tera = match load_template(
                format!("{}**/*.{{{}}}", project_path.to_string_lossy(), template.extensions.as_str()).as_str(),
            ) {
                Ok(t) => t,
                Err(e) => {
                    panic!("unable to load template");
                }
            };
            let mut context = Context::new();
            context.insert("artifact_name", "chaos");
            for name in tera.get_template_names() {
                println!("file {}", name);
                let destFile = format!("{}{}{}", project_path.to_string_lossy(), "/", name);
                let replacer = tera.render(name, &context).unwrap();
                let mut f = OpenOptions::new().write(true).open(destFile).unwrap();
                let _ = f.write_all(replacer.as_bytes());
                let _ = f.flush();
            }

            // To Do
            //project builder struct builder pattern to keep project details
        }
    }
}

fn load_template(path: &str) -> Result<Tera, Error> {
    let mut tera = match Tera::new(path) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            return Err(e);
        }
    };

    tera.autoescape_on(vec![".html"]);
    Ok(tera)
}

fn check_prerequisites() {
    match Command::new("git").stdout(Stdio::null()).spawn() {
        Ok(_) => {}
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("`git` was not found! please check it is installed")
            } else {
                println!("Some strange error occurred :(");
            }
        }
    }
}
