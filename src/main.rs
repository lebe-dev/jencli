use std::path::Path;
use std::process::exit;
use anyhow::Context;
use clap::{Arg, ArgAction, Command};
use reqwest::blocking::ClientBuilder;
use crate::config::load_config_from_file;
use crate::jenkins::list::{get_jenkins_job_list, JenkinsJob};
use crate::jenkins::rebuild::rebuild_job;
use crate::logging::get_logging_config;

pub mod logging;
pub mod config;
pub mod jenkins;

const LIST_COMMAND: &str = "list";
const MASK_ARG: &str = "mask";

const REBUILD_COMMAND: &str = "rebuild";
const NAME_ARG: &str = "name";

const EXIT_CODE: i32 = 1;

fn main() {
    let matches = Command::new("jencli")
        .about("cli for jenkins")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(LIST_COMMAND)
                .short_flag('l')
                .long_flag(LIST_COMMAND)
                .about("list jobs")
                .arg(
                    Arg::new(MASK_ARG)
                        .long(MASK_ARG)
                        .help("filter jobs by mask")
                        .required(false)
                        .action(ArgAction::Set)
                )
        )
        .subcommand(
            Command::new(REBUILD_COMMAND)
                .short_flag('r')
                .long_flag(REBUILD_COMMAND)
                .about("start rebuild for job")
                .arg(
                    Arg::new(NAME_ARG)
                        .short('n')
                        .long(NAME_ARG)
                        .action(ArgAction::Set)
                        .required(true)
                        .help("job name"),
                )
        )
        .get_matches();

    init_logging("info").expect("unable to init logging subsystem");

    match matches.subcommand() {
        Some((LIST_COMMAND, list_matches)) => {
            let config_file_path = Path::new("config.yml");

            let config = load_config_from_file(&config_file_path).expect("unable to load config from file");

            let client = ClientBuilder::new().build()
                .expect("unable to build http client");

            match get_jenkins_job_list(&client, &config.jenkins_url,
                                       &config.username, &config.token) {
                Ok(job_list) => {

                    if let Some(mask) = list_matches
                        .get_one::<String>(MASK_ARG) {

                        let job_list = job_list.into_iter()
                            .filter(|j|
                            j.name.to_lowercase().contains(mask)).collect::<Vec<JenkinsJob>>();

                        let json = serde_json::to_string(&job_list).expect("unable to serialize results");

                        println!("{json}");

                    } else {

                        let json = serde_json::to_string(&job_list).expect("unable to serialize results");

                        println!("{json}");
                    }

                }
                Err(e) => {
                    eprintln!("error: {}", e);
                    eprintln!("{}", e.root_cause());
                    exit(EXIT_CODE);
                }
            }
        }
        Some((REBUILD_COMMAND, rebuild_matches)) => {
            if let Some(job_name) = rebuild_matches.get_one::<String>(NAME_ARG) {
                println!("rebuilding job '{NAME_ARG}'...");

                let config_file_path = Path::new("config.yml");

                let config = load_config_from_file(&config_file_path).expect("unable to load config from file");

                let client = ClientBuilder::new().build()
                    .expect("unable to build http client");

                match rebuild_job(&client, &config.jenkins_url, &config.username, &config.token, job_name) {
                    Ok(_) => println!("rebuild successfully executed"),
                    Err(e) => {
                        eprintln!("error: {}", e);
                        eprintln!("{}", e.root_cause());
                        exit(EXIT_CODE);
                    }
                }

            } else {
                eprintln!("Required '{NAME_ARG}' argument");
                exit(EXIT_CODE);
            }
        }
        _ => {}
    }
}

fn init_logging(logging_level: &str) -> anyhow::Result<()> {
    let logging_config = get_logging_config(logging_level);
    log4rs::init_config(logging_config).context("unable to init logging subsystem")?;
    Ok(())
}
