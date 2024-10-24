mod cli;
mod io;
mod time;

use std::{
    env,
    fs::{self, File},
    process,
};

use clap::Parser;
use toml::Table;

use cli::{Cli, Command, Data, Session};
use io::input;

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
            println!("{project}: {new_hours}", new_hours = time::fmt(new_hours));
            data.hours.insert(project, new_hours);
        }
        Command::List { raw } => {
            if data.hours.is_empty() {
                println!("No data found");
                process::exit(0);
            }
            for (key, value) in data.hours.iter() {
                if raw {
                    println!("{key}: {value}");
                } else {
                    let longest_key = data.hours.keys().map(|k| k.len()).max().unwrap();
                    let space = " ".repeat(longest_key - key.len() + 2);
                    let value = time::fmt(*value);
                    println!("{key}:{space}{value}",);
                }
            }
        }
        Command::Start { project } => match data.session {
            Some(session) => {
                let elapsed = (time::now() - session.start) as f32 / 3600.0;
                let new_val = data.hours.get(&session.key).unwrap_or(&0.0) + elapsed;

                println!(
                    "Session ended - {key} [updated: {value}]",
                    key = session.key,
                    value = time::fmt(new_val)
                );
                *data.hours.entry(session.key).or_insert(new_val) += elapsed;

                let hours = *data.hours.get(&project).unwrap_or(&0.0);
                println!(
                    "Session started - {key} [current: {value}]",
                    key = project,
                    value = time::fmt(hours)
                );
                data.session = Some(Session::new(project));
            }
            None => {
                let hours = time::fmt(*data.hours.get(&project).unwrap_or(&0.0));
                println!("Session started - {project} [current: {hours}]",);
                data.session = Some(Session::new(project));
            }
        },
        Command::End => match data.session {
            Some(session) => {
                let elapsed = (time::now() - session.start) as f32 / 3600.0;
                let new_val = data.hours.get(&session.key).unwrap_or(&0.0) + elapsed;

                println!(
                    "Session ended - {key} [updated: {value}]",
                    key = session.key,
                    value = time::fmt(new_val)
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
                let elapsed = (time::now() - session.start) as f32 / 3600.0;
                let total = stored + elapsed;

                println!(
                    "{key} \
                        \n{divider} \
                        \nStored:   {stored} \
                        \nElapsed:  {elapsed} \
                        \nTotal:    {total}",
                    key = session.key,
                    divider = "-".repeat(usize::max(18, session.key.len())),
                    stored = time::fmt(stored),
                    elapsed = time::fmt(elapsed),
                    total = time::fmt(total)
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
            print!("Are you sure you want to clear all data? (Y/n): ");
            match input().as_str() {
                "yes" | "y" | "" => {
                    data.hours.clear();
                    println!("Data cleared");
                }
                _ => println!("Clear aborted"),
            }
        }
    }

    let contents = toml::to_string(&data).unwrap();
    fs::write(&path, contents).unwrap();
}
