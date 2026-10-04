use dashmap::DashMap;
use pulldown_cmark::{Event, Parser, Tag, TagEnd, TextMergeStream};
use std::{
    collections::{HashMap, HashSet},
    fs, io,
    path::PathBuf,
};

use crate::utils::create_options;

pub type SectionLinkMap = DashMap<PathBuf, HashSet<String>>;

/// Scan markdown file and collect section links based on its heading.
/// # Errors
/// This function will return an error if `path` does not already exist.
pub fn parse_file_headings(path: &PathBuf) -> io::Result<HashSet<String>> {
    fs::read_to_string(path)
        .map(|content| crate::parser::collect_heading_links(&content))
}

/// Collect section links from markdown content based on headings using
/// [GitHub Flavored Markdown (GFM)](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#section-links)
/// rules.
///
/// [Custom anchors](https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#custom-anchors)
/// are not supported so far.
///
/// # Examples
///
/// ```rust
/// use mdrefcheck::parser::collect_heading_links;
///
/// let input = "# Intro\n# Intro\n## Hello, World!";
/// let anchors = collect_heading_links(input);
///
/// assert!(anchors.contains("intro"));
/// assert!(anchors.contains("intro-1"));
/// assert!(anchors.contains("hello-world"));
///
/// // a heading whose text collides with a generated anchor gets a further suffix
/// let anchors = collect_heading_links("# a\n# a\n# a-1");
/// assert!(anchors.contains("a-1-1"));
/// ```
#[must_use]
pub fn collect_heading_links(content: &str) -> HashSet<String> {
    let mut headings = HashSet::new();
    let mut heading_counter = HashMap::new();
    let parser = TextMergeStream::new(Parser::new_ext(content, create_options()));
    let mut current_heading = String::new();
    let mut in_heading = false;

    for event in parser {
        match event {
            Event::Start(Tag::Heading { .. }) => {
                in_heading = true;
                current_heading.clear();
            }
            Event::Text(text) | Event::Code(text) if in_heading => {
                current_heading.push_str(&text);
            }
            Event::End(TagEnd::Heading { .. }) => {
                // Same as github-slugger: bump the suffix until the anchor is unused,
                // so "a", "a", "a-1" gives "a", "a-1", "a-1-1"
                let base_link = heading2link(&current_heading);
                let mut link = base_link.clone();
                while headings.contains(&link) {
                    let counter = heading_counter.entry(base_link.clone()).or_insert(0);
                    *counter += 1;
                    link = format!("{base_link}-{counter}");
                }
                headings.insert(link);
                in_heading = false;
            }
            _ => {}
        }
    }
    headings
}

/// Convert heading text to a GFM-style anchor string.
///
/// Does not deduplicate - see `collect_heading_links` for counter logic.
///
/// # Examples
///
/// ```rust
/// use mdrefcheck::parser::heading2link;
///
/// assert_eq!(heading2link("Hello World"), "hello-world");
/// assert_eq!(heading2link("This -- Is__A_Test!"), "this----is__a_test");
/// assert_eq!(heading2link("A heading with 💡 emoji!"), "a-heading-with--emoji");
/// ```
#[must_use]
pub fn heading2link(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() || c == '-' || c == '_' {
                Some(c)
            } else if c.is_whitespace() {
                Some('-')
            } else {
                None
            }
        })
        .collect()
}
