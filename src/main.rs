use clap::Parser;
use colored::Colorize;
use mdrefcheck::config::CliConfig;
use mdrefcheck::parser::SectionLinkMap;
use mdrefcheck::scanner::gather_markdown_files;
use mdrefcheck::{checks::run_checks, utils::create_file_set};
use std::{fs, process};

fn main() {
    let config = CliConfig::parse();

    let exclude_paths = create_file_set(&config.exclude);

    let files = gather_markdown_files(&config.paths, &exclude_paths);
    let mut section_links = SectionLinkMap::new();

    let mut has_errors = false;

    for (path, content) in files
        .iter()
        .filter_map(|p| fs::read_to_string(p).ok().map(|c| (p, c)))
    {
        let errors = run_checks(&content, path, &mut section_links, &config);
        for err in &errors {
            println!("{}", err);
        }
        if !errors.is_empty() {
            has_errors = true;
        }
    }

    // eprintln!("{:#?}", section_links);

    if has_errors {
        process::exit(1);
    }

    println!("{}", "No broken references found.".green())
}
