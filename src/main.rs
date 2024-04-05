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
    key: Option<String>,

    #[arg(short, long, default_value_t = 1, allow_hyphen_values = true)]
    num: i64,
}

const HOME: &str = env!("HOME");

fn main() {
    let args = Args::parse();
    let path = format!("{HOME}/hours.toml");

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

    match args.key {
        Some(key) => match data.get(&key) {
            Some(value) => {
                data.insert(key, Value::Integer(value.as_integer().unwrap() + args.num));
                let toml = toml::to_string(&data).unwrap();
                fs::write(&path, toml).unwrap();
            }
            None => {
                data.insert(key, Value::Integer(args.num));
                let toml = toml::to_string(&data).unwrap();
                fs::write(&path, toml).unwrap();
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
