use std::path::PathBuf;

use glob::glob;

use crate::{
    config::{FilterOpts, SymlinkOption},
    file_handling::FileInfo,
};

pub fn read_glob_entries<'a>(
    opts: &'a FilterOpts,
    pattern: glob::Pattern,
) -> impl Iterator<Item = FileInfo> + 'a {
    glob(pattern.as_str())
        .expect(&format!("Invalid glob pattern: {}", &pattern))
        .map(move |res| match res {
            Ok(path) => path,
            Err(e) => panic!("Error reading file info: {}", e),
        })
        .filter_map(move |path: PathBuf| match path.symlink_metadata() {
            Ok(sm) => match (opts.symlinks, sm.is_symlink()) {
                (SymlinkOption::Omit, true) => None,
                (SymlinkOption::Include, true) => Some(FileInfo::new(path, sm, opts)),
                (_, _) => match path.metadata() {
                    Ok(m) => Some(FileInfo::new(path, m, opts)),
                    Err(e) => panic!("Unable to read metadata for {:?}: {}", path, e),
                },
            },
            Err(e) => panic!("Unable to read symlink metadata for {:?}: {}", path, e),
        })
}
