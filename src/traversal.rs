use std::{path::PathBuf, time::SystemTime};

use glob::{glob, Pattern};

use crate::{
    config::{FilterOpts, SymlinkOption},
    file_handling::FileInfo,
};

/// Iterates over the entries implied by a glob and constructs [FileInfo] for each.
///
/// Panics at any error in this process, since it's better for the script to be trustworthy than to
/// try to paper over a filesystem problem (which is above its pay grade).
pub fn read_glob_entries<'a>(
    opts: &'a FilterOpts,
    pattern: &'a glob::Pattern,
) -> impl Iterator<Item = FileInfo<'a>> {
    glob(pattern.as_str())
        .unwrap_or_else(|_| panic!("Invalid glob pattern: {pattern}"))
        .map(move |res| match res {
            Ok(path) => path,
            Err(e) => panic!("Error reading file info: {e}"),
        })
        .filter_map(move |path: PathBuf| match path.symlink_metadata() {
            Ok(sm) => match (opts.symlinks, sm.is_symlink()) {
                (SymlinkOption::Omit, true) => None,
                (SymlinkOption::Include, true) => Some(FileInfo::new(path, sm, opts)),
                (_, _) => match path.metadata() {
                    Ok(m) => Some(FileInfo::new(path, m, opts)),
                    Err(e) => panic!("Unable to read metadata for {path:?}: {e}"),
                },
            },
            Err(e) => panic!("Unable to read symlink metadata for {path:?}: {e}"),
        })
}

/// Iterates over the files described by a slice of globs and finds the latest modification time
/// ("mtime").
pub fn get_max_mtime<'a>(
    filter_opts: &'a FilterOpts,
    globs: &'a [Pattern],
) -> Option<(FileInfo<'a>, SystemTime)> {
    globs
        .iter()
        .flat_map(|g| read_glob_entries(filter_opts, g))
        .filter(FileInfo::is_relevant)
        .map(|fi| {
            let mtime = fi.modified_time().unwrap();
            (fi, mtime)
        })
        .max_by(|(_, xt), (_, yt)| xt.cmp(yt))
}
