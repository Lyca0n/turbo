#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::ErrorKind;
use std::process::Command;
use std::process::Stdio;

use clap::Parser;
use inquire::Confirm;
use inquire::Select;
use inquire::Text;
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
        Commands::New => {
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

            println!("using template {:#?}", template);
            //try tera
            let tera = match load_template(format!("{}**/*.{{{}}}",template.location.as_str(), template.extensions.as_str()).as_str()) {
                Ok(t) => t,
                Err(e) => {
                    panic!("unable to load template");
                }
            };

            for name in tera.get_template_names() {
                println!("file {}", name)
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
