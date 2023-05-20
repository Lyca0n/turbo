#![allow(dead_code)]
#![allow(unused_variables)]

use std::io::ErrorKind;
use std::process::Command;
use std::process::Stdio;

use clap::Parser;
use inquire::Select;
use turbo::cmd::{Cli, Commands};
use turbo::config::BuilderConfig;
static VERSION: &str = "V0.0.1";

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
            .prompt();

            // To Do 
            //project builder struct builder pattern to keep project details
        }
    }
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
