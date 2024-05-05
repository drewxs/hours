use std::{
    fs::{self, File},
    process::exit,
};

use clap::Parser;
use toml::{Table, Value};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    project: Option<String>,

    #[arg(short, long, default_value_t = 1, allow_hyphen_values = true)]
    num: i64,
}

fn main() {
    let args = Args::parse();
    let home = std::env::var("HOME").unwrap();
    let path = format!("{home}/hours.toml");

    let contents = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            File::create(&path).unwrap();
            "".to_string()
        }
    };
    let mut data = match contents.parse::<Table>() {
        Ok(d) => d,
        Err(_) => {
            println!("Error parsing toml file");
            exit(1);
        }
    };

    match args.project {
        Some(project) => match data.get(&project) {
            Some(value) => {
                let new_val = value.as_integer().unwrap() + args.num;
                data.insert(project.clone(), Value::Integer(new_val));
                let toml = toml::to_string(&data).unwrap();
                fs::write(&path, toml).unwrap();
                println!("{project}: {new_val}");
            }
            None => {
                let new_val = args.num;
                data.insert(project.clone(), Value::Integer(new_val));
                let toml = toml::to_string(&data).unwrap();
                fs::write(&path, toml).unwrap();
                println!("{project}: {new_val}");
            }
        },
        None => {
            if data.is_empty() {
                println!("No data found");
                exit(1);
            }

            for (key, value) in data.iter() {
                println!("{key}: {value}");
            }
        }
    }
}
