mod email;
mod image;
mod section;

use pulldown_cmark::{BrokenLink, CowStr, Event, LinkType, Parser, Tag};
use regex::Regex;

use crate::checks::email::validate_email;
use crate::checks::image::validate_image;
use crate::checks::section::validate_section_link;
use crate::config::CliConfig;
use crate::diagnostics::ValidationError;
use crate::parser;
use crate::utils::{compute_line_starts, create_options, offset_to_line_col};
use std::cell::RefCell;
use std::path::Path;
use std::sync::Arc;

/// Dispatch all checks and return errors
#[must_use]
pub fn run_checks(
    content: &str,
    path: &Path,
    section_links: &Arc<parser::SectionLinkMap>,
    config: &CliConfig,
) -> Vec<ValidationError> {
    let errors = RefCell::new(Vec::new());
    let line_starts = compute_line_starts(content);

    if !section_links.contains_key(path) {
        section_links
            .insert(path.to_path_buf(), parser::collect_heading_links(content));
    }

    let callback = |broken: BrokenLink<'_>| {
        if !to_exclude(&broken.reference, &config.ignore) {
            let (line, col) = offset_to_line_col(broken.span.start, &line_starts);
            errors.borrow_mut().push(ValidationError::new(
                path,
                line,
                col,
                format!("Broken link: {}", broken.reference),
            ));
        }
        None::<(CowStr, CowStr)>
    };

    let parser = Parser::new_with_broken_link_callback(
        content,
        create_options(),
        Some(&callback),
    );

    for (event, range) in parser.into_offset_iter() {
        let (line, col) = offset_to_line_col(range.start, &line_starts);

        match event {
            Event::Start(Tag::Link {
                link_type,
                dest_url,
                ..
            }) => match link_type {
                LinkType::Inline if !to_exclude(&dest_url, &config.ignore) => {
                    if let Err(e) = check_inline(path, &dest_url, section_links) {
                        errors
                            .borrow_mut()
                            .push(ValidationError::new(path, line, col, e));
                    }
                }

                LinkType::Email if !to_exclude(&dest_url, &config.ignore) => {
                    if let Err(e) = validate_email(&dest_url) {
                        errors
                            .borrow_mut()
                            .push(ValidationError::new(path, line, col, e));
                    }
                }

                _ => {}
            },

            Event::Start(Tag::Image { dest_url, .. })
                if !to_exclude(&dest_url, &config.ignore) =>
            {
                if let Err(e) = validate_image(path, &dest_url) {
                    errors
                        .borrow_mut()
                        .push(ValidationError::new(path, line, col, e));
                }
            }

            _ => {}
        }
    }

    errors.into_inner()
}

fn check_inline(
    current_path: &Path,
    dest: &str,
    doc_headings: &Arc<parser::SectionLinkMap>,
) -> Result<(), String> {
    if dest.starts_with("http://") || dest.starts_with("https://") {
        return Ok(());
    }

    if let Some(email) = dest.strip_prefix("mailto:") {
        return validate_email(email);
    }

    validate_section_link(current_path, dest, doc_headings)
}

fn to_exclude(dest: &str, exclude_link_regexes: &[Regex]) -> bool {
    exclude_link_regexes
        .iter()
        .any(|re| re.is_match(dest.as_ref()))
}
