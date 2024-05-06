use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    process,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use serde::{Deserialize, Serialize};
use toml::Table;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
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

    #[arg(short, long, help = "List all hours")]
    list: bool,

    #[arg(short, long, help = "Clear")]
    clear: bool,
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
    let path = env::var("HOURS_PATH")
        .unwrap_or_else(|_| format!("{}/hours.toml", env::var("HOME").unwrap()));

    let contents = fs::read_to_string(&path).unwrap_or_else(|_| {
        File::create(&path).unwrap();
        "".to_string()
    });

    // TODO: create a way to fix broken toml files
    let mut data: Data = contents
        .parse::<Table>()
        .expect("Invalid TOML file")
        .try_into()
        .expect("Error parsing");

    if args.list {
        if data.hours.is_empty() {
            println!("No data found");
        }
        for (k, v) in data.hours.iter() {
            println!("{k}: {v}");
        }
        process::exit(0);
    }

    if args.end {
        if data.session.is_none() {
            eprintln!("No session started");
            process::exit(1);
        }

        let session = data.session.unwrap();
        let project = session.key;
        let elapsed = (now() - session.start) as f32 / 3600.0;
        let new_val = data.hours.get(&project).unwrap_or(&0.0) + elapsed;

        println!("Session ended - {project}:{new_val}");
        *data.hours.entry(project).or_insert(new_val) += elapsed;
        data.session = None;

        save(path, data);
        process::exit(0);
    }

    if args.clear {
        data.hours.clear();
        save(path, data);
        process::exit(0);
    }

    let project = args.project.unwrap_or_else(|| {
        eprintln!("Project key required (e.g. 'hours -p my_project')");
        process::exit(1);
    });

    if args.start {
        if data.session.is_some() {
            eprintln!("Session already started");
            process::exit(1);
        }

        println!("Session started - {project}");
        data.session = Some(Session {
            key: project,
            start: now(),
        });

        save(path, data);
        process::exit(0);
    }

    let hours = data.hours.get(&project).unwrap_or(&0.0);
    let new_val = hours + args.num;
    println!("{project}: {new_val}");
    data.hours.insert(project, new_val);

    save(path, data);
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
