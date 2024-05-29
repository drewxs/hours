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
    about = "Time tracking CLI", long_about = None,
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
    List {
        #[arg(short, long)]
        #[arg(help = "List raw data")]
        raw: bool,
    },

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
    #[command(about = "Start/switch sessions")]
    Start { project: String },

    #[command(visible_alias = "e")]
    #[command(about = "End current session")]
    End,

    #[command(visible_alias = "v")]
    #[command(about = "View current session")]
    View,

    #[command(visible_alias = "rm")]
    #[command(about = "Remove hours")]
    Remove {
        #[arg(index = 1)]
        #[arg(help = "Project key")]
        project: String,
    },

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

impl Session {
    fn new(key: String) -> Self {
        Self { key, start: now() }
    }
}

fn main() {
    let args = Cli::parse();
    let path = env::var("HOURS_PATH")
        .unwrap_or_else(|_| format!("{}/hours.toml", env::var("HOME").unwrap()));

    let contents = fs::read_to_string(&path).unwrap_or_else(|_| {
        File::create(&path).unwrap();
        "[hours]".to_string()
    });

    let mut data: Data = contents
        .parse::<Table>()
        .expect("Invalid TOML file")
        .try_into()
        .expect("Error parsing");

    match args.cmd {
        Command::Add { project, hours } => {
            let new_hours = data.hours.get(&project).unwrap_or(&0.0) + hours;
            println!("{project}: {new_hours}", new_hours = time_str(new_hours));
            data.hours.insert(project, new_hours);
        }
        Command::List { raw } => {
            if data.hours.is_empty() {
                println!("No data found");
                process::exit(0);
            }
            for (key, value) in data.hours.iter() {
                let longest_key = data.hours.keys().map(|k| k.len()).max().unwrap();
                match raw {
                    true => println!("{key}: {value}"),
                    false => println!(
                        "{key}:{space}{value}",
                        space = " ".repeat(longest_key - key.len() + 2),
                        value = time_str(*value)
                    ),
                }
            }
        }
        Command::Start { project } => match data.session {
            Some(session) => {
                let elapsed = (now() - session.start) as f32 / 3600.0;
                let new_val = data.hours.get(&session.key).unwrap_or(&0.0) + elapsed;

                println!(
                    "Session ended - {key} [updated: {value}]",
                    key = session.key,
                    value = time_str(new_val)
                );
                *data.hours.entry(session.key).or_insert(new_val) += elapsed;

                let hours = *data.hours.get(&project).unwrap_or(&0.0);
                println!(
                    "Session started - {key} [current: {value}]",
                    key = project,
                    value = time_str(hours)
                );
                data.session = Some(Session::new(project));
            }
            None => {
                let hours = *data.hours.get(&project).unwrap_or(&0.0);
                println!(
                    "Session started - {project} [current: {value}]",
                    value = time_str(hours)
                );
                data.session = Some(Session::new(project));
            }
        },
        Command::End => match data.session {
            Some(session) => {
                let elapsed = (now() - session.start) as f32 / 3600.0;
                let new_val = data.hours.get(&session.key).unwrap_or(&0.0) + elapsed;

                println!(
                    "Session ended - {key} [updated: {value}]",
                    key = session.key,
                    value = time_str(new_val)
                );
                *data.hours.entry(session.key).or_insert(new_val) += elapsed;
                data.session = None;
            }
            None => {
                eprintln!("No session started");
                process::exit(1);
            }
        },
        Command::View => match data.session {
            Some(session) => {
                let stored = *data.hours.get(&session.key).unwrap_or(&0.0);
                let elapsed = (now() - session.start) as f32 / 3600.0;
                let total = stored + elapsed;

                println!(
                    "{key} \
                        \n{divider} \
                        \nStored:   {stored} \
                        \nElapsed:  {elapsed} \
                        \nTotal:    {total}",
                    key = session.key,
                    divider = "-".repeat(usize::max(18, session.key.len())),
                    stored = time_str(stored),
                    elapsed = time_str(elapsed),
                    total = time_str(total)
                );
                process::exit(0);
            }
            None => {
                eprintln!("No session running");
                process::exit(1);
            }
        },
        Command::Remove { project } => {
            if !data.hours.contains_key(&project) {
                eprintln!("{project} not found");
                process::exit(1);
            }
            data.hours.remove(&project);
            println!("Removed {project}");
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

fn time_str(time: f32) -> String {
    let hours = time as u32;
    let minutes = ((time - hours as f32) * 60.0) as u32;
    let seconds = (((time - hours as f32) * 60.0 - minutes as f32) * 60.0) as u32;
    String::from(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
}
