use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
pub struct ArchVileArgs {
    #[arg(short = 'u', long = "url")]
    pub url: String,

    #[arg(short = 'H', long = "header", action = clap::ArgAction::Append)]
    pub headers: Vec<String>,

    #[arg(short = 'f', long = "file")]
    pub file: PathBuf,

    #[arg(short = 'F', long = "field", default_value = "file")]
    pub field: String,

    #[arg(short = 'c', long = "connection", default_value_t = 1)]
    pub connections: usize,
}
