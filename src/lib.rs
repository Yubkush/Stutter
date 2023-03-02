use clap::Parser;
use std::fs::{self, DirEntry};
use std::io::Error;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub struct Cli {
    /// Case sensitive search in English. Default: false
    #[arg(short, default_value_t = false)]
    sensitive: bool,

    /// Search in English. Default: false
    #[arg(short, default_value_t = false)]
    english: bool,

    /// The pattern to search for
    pattern: String,

    /// The path to the files to read
    path: PathBuf,
}

pub fn read(cli: &Cli) {
    match fs::metadata(&cli.path).unwrap().is_dir() {
        true => search_multiple_files(cli),
        false => search_file(cli),
    }
}

pub fn search_multiple_files(cli: &Cli) {
    let paths: Vec<Result<DirEntry, Error>> = fs::read_dir(&cli.path)
        .unwrap()
        .filter(|path| path.as_ref().unwrap().path().extension().is_some())
        .filter(|path| path.as_ref().unwrap().path().extension().unwrap() == "pdf")
        .collect();

    for path in paths {
        let path = path.unwrap().path();
        let content = pdf_extract::extract_text(&path).unwrap();
        let result = if !cli.english {
            let pattern = cli.pattern.chars().rev().collect::<String>();
            search_case_sensitive(&pattern, &content)
        } else if cli.sensitive {
            search_case_sensitive(&cli.pattern, &content)
        } else {
            search_case_insensitive(&cli.pattern, &content)
        };

        for line in result.iter() {
            let path_str = path.to_str().unwrap();
            println!("{}: {}", &path_str, line);
        }
    }
}

pub fn search_file(cli: &Cli) {
    let content = pdf_extract::extract_text(&cli.path).unwrap();
    let result = if !cli.english {
        let pattern = cli.pattern.chars().rev().collect::<String>();
        search_case_sensitive(&pattern, &content)
    } else if cli.sensitive {
        search_case_sensitive(&cli.pattern, &content)
    } else {
        search_case_insensitive(&cli.pattern, &content)
    };

    for line in result.iter() {
        let path_str = cli.path.to_str().unwrap();
        println!("{}: {}", &path_str, line);
    }
}

pub fn search_case_sensitive<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn search_case_insensitive<'a>(pattern: &str, content: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();

    content
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern))
        .collect()
}
