use clap::Parser;
use colored::Colorize;
use mdrefcheck::checks::run_checks;
use mdrefcheck::config::CliConfig;
use mdrefcheck::parser::SectionLinkMap;
use mdrefcheck::scanner::gather_markdown_files;
use rayon::prelude::*;
use std::sync::Arc;
use std::{fs, process, time::Instant};

fn main() {
    let config = CliConfig::parse();
    let start_time = Instant::now();

    env_logger::Builder::new()
        // .filter_level(log::LevelFilter::Warn)
        .filter_module("mdrefcheck", config.verbosity.log_level_filter())
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let files = gather_markdown_files(&config.paths, &config.exclude, config.no_ignore);

    println!(
        "File scan completed in {:.2?}, {} md files gathered",
        start_time.elapsed(),
        files.len()
    );

    let section_links = Arc::new(SectionLinkMap::new());

    let mut all_errors: Vec<_> = files
        .par_iter()
        .filter_map(|path| {
            let content = fs::read_to_string(path).ok()?;

            let errors = run_checks(&content, path, &section_links, &config);

            if errors.is_empty() {
                None
            } else {
                Some(errors)
            }
        })
        .flatten()
        .collect();

    all_errors.sort_by(|a, b| {
        a.path
            .cmp(&b.path)
            .then_with(|| a.line.cmp(&b.line))
            .then_with(|| a.col.cmp(&b.col))
    });

    if !all_errors.is_empty() {
        for err in &all_errors {
            println!("{err}");
        }
        println!("Completed in {:.2?}", start_time.elapsed());
        process::exit(1);
    }

    println!("Completed in {:.2?}", start_time.elapsed());
    println!("{}", "No broken references found.".green());
}
