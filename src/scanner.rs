use ignore::{WalkBuilder, overrides::OverrideBuilder, types::TypesBuilder};
use log::{debug, error, warn};
use path_clean::PathClean;
use std::{collections::HashSet, path::PathBuf};

/// Gathers Markdown files recursively under the given paths.
#[must_use]
pub fn gather_markdown_files(
    paths: &[PathBuf],
    exclude_paths: &[PathBuf],
    no_ignore: bool,
) -> Vec<PathBuf> {
    if paths.is_empty() {
        warn!("No paths provided to scan.");
        return vec![];
    }

    let types = match TypesBuilder::new()
        .add_defaults()
        .select("markdown")
        .build()
    {
        Ok(t) => t,
        Err(e) => {
            error!("Failed to build markdown filter: {e}");
            return vec![];
        }
    };

    let overrides = {
        let mut ob = OverrideBuilder::new(".");

        for path in exclude_paths {
            // Convert to string and normalize slashes for the glob
            let glob_str = path
                .clean()
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR, "/");

            // Add '!' to make it an *ignore* pattern
            let ignore_glob = format!("!{glob_str}");
            debug!("Adding exclude rule: {ignore_glob}");

            if let Err(e) = ob.add(&ignore_glob) {
                warn!("Invalid exclude pattern '{}': {}", path.display(), e);
            }
        }

        match ob.build() {
            Ok(o) => o,
            Err(e) => {
                error!("Failed to build exclude override rules: {e}");
                return vec![];
            }
        }
    };

    // Pre-filtering the initial input paths
    // https://github.com/BurntSushi/ripgrep/issues/2986

    let exclude_set: HashSet<PathBuf> =
        exclude_paths.iter().map(PathClean::clean).collect();

    let filtered_paths: Vec<PathBuf> = paths
        .iter()
        .filter(|path| {
            let clean_path = path.clean();
            let is_excluded = clean_path.ancestors().any(|a| exclude_set.contains(a));

            if is_excluded {
                debug!("Excluding root path: {}", path.display());
            }
            !is_excluded
        })
        .cloned()
        .collect();

    if filtered_paths.is_empty() {
        debug!("All input paths were excluded or empty.");
        return vec![];
    }

    let walker = {
        let mut wb = WalkBuilder::new(&filtered_paths[0]);
        for path in &filtered_paths[1..] {
            wb.add(path);
        }
        wb.standard_filters(!no_ignore)
            .types(types)
            .overrides(overrides)
            .build()
    };

    walker
        .filter_map(|entry_result| match entry_result {
            Ok(entry) => Some(entry),
            Err(e) => {
                warn!("Error scanning path: {e}");
                None
            }
        })
        .filter(|entry| entry.file_type().is_some_and(|ft| ft.is_file()))
        .map(|entry| entry.path().to_path_buf())
        .collect()
}
