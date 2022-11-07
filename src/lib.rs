use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    #[arg(short,long)]
    sensitive: bool,
    
    pub pattern: String,
    pub path: PathBuf,
}

pub fn read(cli: &Cli) {
    let content = pdf_extract::extract_text(&cli.path).unwrap();
    let result = if !cli.sensitive {
        search_case_insensitive(&cli.pattern, &content)
    }
    else {
        search_case_sensitive(&cli.pattern, &content)
    };

    for line in result.iter() {
        println!("{:?}: {}", &cli.path, line);
    }
}

pub fn search_case_sensitive<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    content.
        lines().
        filter(|line| line.contains(pattern)).
        collect()
}

pub fn search_case_insensitive<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();

    content.
        lines().
        filter(|line| line.to_lowercase().contains(&pattern)).
        collect()
}