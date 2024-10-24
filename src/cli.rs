use std::collections::HashMap;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use crate::time;

#[derive(Debug, Parser)]
#[command(
    name = "hours",
    version,
    author = "Andrew X. Shah, drewshah0@gmail.com",
    about = "Time tracking CLI", long_about = None,
    arg_required_else_help = true,
)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
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
pub struct Data {
    pub hours: HashMap<String, f32>,
    pub session: Option<Session>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Session {
    pub key: String,
    pub start: u64,
}

impl Session {
    pub fn new(key: String) -> Self {
        Self {
            key,
            start: time::now(),
        }
    }
}
