use std::{
    collections::HashMap,
    fs::{self, File},
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Project key")]
    project: Option<String>,

    #[arg(short, long, action, help = "Start a session")]
    start: bool,

    #[arg(short, long, action, help = "End current session")]
    end: bool,

    #[arg(
        short,
        long,
        default_value_t = 1.0,
        allow_hyphen_values = true,
        help = "Number of hours"
    )]
    num: f32,
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
    let args = Args::parse();
    let path = std::env::var("HOURS_PATH")
        .unwrap_or_else(|_| format!("{}/hours.toml", std::env::var("HOME").unwrap()));

    let contents = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            File::create(&path).unwrap();
            "".to_string()
        }
    };

    // TODO: create a way to fix broken toml files
    let mut data: Data = contents
        .parse::<Table>()
        .expect("Invalid TOML file")
        .try_into()
        .expect("Error parsing");

    if args.end {
        if data.session.is_none() {
            eprintln!("No session started");
            std::process::exit(1);
        }

        let session = data.session.unwrap();
        let project = session.key;
        let elapsed = (now() - session.start) as f32 / 3600.0;
        let new_val = data.hours.get(&project).unwrap_or(&0.0) + elapsed;

        println!("Session ended - {project}:{new_val}");
        *data.hours.entry(project).or_insert(new_val) += elapsed;
        data.session = None;

        save(path, data);
        return;
    }

    match args.project {
        Some(project) => {
            if args.start {
                if data.session.is_some() {
                    eprintln!("Session already started");
                    std::process::exit(1);
                }

                println!("Session started - {project}");
                data.session = Some(Session {
                    key: project,
                    start: now(),
                });

                save(path, data);
                return;
            }

            let hours = data.hours.get(&project).unwrap_or(&0.0);
            let new_val = hours + args.num;
            println!("{project}: {new_val}");
            data.hours.insert(project, new_val);

            save(path, data);
        }
        None => {
            if data.hours.is_empty() {
                println!("No data found");
            }

            for (key, value) in data.hours.iter() {
                println!("{key}: {value}");
            }
        }
    }
}

fn save(path: String, data: Data) {
    let toml = toml::to_string(&data).unwrap();
    fs::write(&path, toml).unwrap();
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

