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
    #[arg(short, long)]
    project: Option<String>,

    #[arg(short, long, default_value_t = false)]
    start: bool,

    #[arg(short, long)]
    end: bool,

    #[arg(short, long, default_value_t = 1.0, allow_hyphen_values = true)]
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

    match args.project {
        Some(project) => {
            if args.start {
                if data.session.is_some() {
                    eprintln!("Session already started");
                    std::process::exit(1);
                }

                data.session = Some(Session {
                    key: project.clone(),
                    start: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                });

                save(path, data);

                println!("Session started - {project}");
                return;
            }
            if args.end {
                if data.session.is_none() {
                    eprintln!("No session started");
                    std::process::exit(1);
                }

                let session = data.session.clone().unwrap();
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let elapsed = (now - session.start) as f32 / 3600.0;
                let new_val = data.hours.get(&project).unwrap_or(&0.0) + elapsed;
                *data.hours.entry(project.clone()).or_insert(new_val) += elapsed;
                data.session = None;

                save(path, data);

                println!("Session ended - {project}:{new_val}");
                return;
            }

            match data.hours.get(&project) {
                Some(value) => {
                    let new_val = value + args.num;
                    data.hours.insert(project.clone(), new_val);

                    save(path, data);

                    println!("{project}: {new_val}");
                }
                None => {
                    let new_val = args.num;
                    data.hours.insert(project.clone(), new_val);

                    save(path, data);

                    println!("{project}: {new_val}");
                }
            }
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
