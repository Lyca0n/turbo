#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs;
use std::io::ErrorKind;
use std::path::Path;
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
use turbo::renderer::Renderer;
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
            let _ = template.clone_source(project_path);

            // renderer
            let mut renderer = Renderer::new(template.extensions.clone(), name.clone());
            renderer.add_to_context("artifact_name", "chaos");
            renderer.add_to_context("i18n", "true");
            if let Err(r) = renderer.init_template() {
                println!("unable to init template files");
            }
            if let Ok(r) = renderer.render_all_in_place() {
                println!("project created and ready to use!");
            }
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
