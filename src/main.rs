use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    process,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Debug, Parser)]
#[command(
    name = "hours",
    version,
    author = "Andrew X. Shah, drewshah0@gmail.com",
    about = "Hours tracking CLI", long_about = None,
    arg_required_else_help = true,
)]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(visible_alias = "l")]
    #[command(about = "List all hours")]
    List,

    #[command(visible_alias = "a")]
    #[command(about = "Add hours")]
    Add {
        #[arg(index = 1)]
        #[arg(help = "Project key")]
        project: String,

        #[arg(index = 2, allow_hyphen_values = true)]
        #[arg(help = "Number of hours")]
        hours: f32,
    },

    #[command(visible_alias = "s")]
    #[command(about = "Start a session")]
    Start {
        #[arg(index = 1)]
        #[arg(help = "Project key")]
        project: String,
    },

    #[command(visible_alias = "sw")]
    Switch {
        #[arg(index = 1)]
        #[arg(help = "Project key")]
        project: String,
    },

    #[command(visible_alias = "e")]
    #[command(about = "End current session")]
    End,

    #[command(visible_alias = "c")]
    #[command(about = "Clear ")]
    Clear,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Data {
    hours: HashMap<String, f32>,
    session: Option<Session>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Session {
    key: String,
    start: u64,
}

fn main() {
    let args = Cli::parse();
    let path = env::var("HOURS_PATH")
        .unwrap_or_else(|_| format!("{}/hours.toml", env::var("HOME").unwrap()));

    let contents = fs::read_to_string(&path).unwrap_or_else(|_| {
        File::create(&path).unwrap();
        "[hours]".to_string()
    });

    // TODO: create a way to fix broken toml files
    let mut data: Data = contents
        .parse::<Table>()
        .expect("Invalid TOML file")
        .try_into()
        .expect("Error parsing");

    match args.cmd {
        Command::Add { project, hours } => {
            let new_hours = data.hours.get(&project).unwrap_or(&0.0) + hours;
            println!("{project}: {new_hours}");
            data.hours.insert(project, new_hours);
        }
        Command::List => {
            if data.hours.is_empty() {
                println!("No data found");
            }
            for (k, v) in data.hours.iter() {
                println!("{k}: {v}");
            }
        }
        Command::Start { project } => {
            if let Some(session) = data.session {
                eprintln!("A session already exists: {}", session.key);
                process::exit(1);
            }

            let hours = data.hours.get(&project).unwrap_or(&0.0);
            println!("Session started - {project} [current: {hours} hours]",);
            data.session = Some(Session {
                key: project,
                start: now(),
            });
        }
        Command::Switch { project } => {
            if data.session.is_none() {
                eprintln!("No session started");
                process::exit(1);
            }

            let session = data.session.unwrap();
            let elapsed = (now() - session.start) as f32 / 3600.0;
            let new_val = data.hours.get(&session.key).unwrap_or(&0.0) + elapsed;

            println!(
                "Session ended - {} [updated: {} hours]",
                session.key, new_val
            );
            *data.hours.entry(session.key).or_insert(new_val) += elapsed;

            let hours = data.hours.get(&project).unwrap_or(&0.0);
            println!("Session started - {project} [current: {hours} hours]",);
            data.session = Some(Session {
                key: project,
                start: now(),
            });
        }
        Command::End => {
            if data.session.is_none() {
                eprintln!("No session started");
                process::exit(1);
            }

            let session = data.session.unwrap();
            let elapsed = (now() - session.start) as f32 / 3600.0;
            let new_val = data.hours.get(&session.key).unwrap_or(&0.0) + elapsed;

            println!(
                "Session ended - {} [updated: {} hours]",
                session.key, new_val
            );
            *data.hours.entry(session.key).or_insert(new_val) += elapsed;
            data.session = None;
        }
        Command::Clear => {
            data.hours.clear();
            println!("Data cleared");
        }
    }

    let contents = toml::to_string(&data).unwrap();
    fs::write(&path, contents).unwrap();
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
