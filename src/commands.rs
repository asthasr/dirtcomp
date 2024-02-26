use std::process::exit;

use glob::Pattern;

use crate::{config::FilterOpts, file_handling::FileInfo, traversal::read_glob_entries};

fn get_max_mtime<'a>(filter_opts: &'a FilterOpts, globs: Vec<Pattern>) -> Option<FileInfo<'a>> {
    globs
        .into_iter()
        .flat_map(|g| read_glob_entries(filter_opts, g))
        .filter(FileInfo::is_relevant)
        .max_by_key(FileInfo::modified_time)
}

pub fn simple_check(filter_opts: FilterOpts, base: Pattern, target: Pattern) {
    let base = get_max_mtime(&filter_opts, vec![base]).expect("Nothing matched the base glob!");

    let target =
        get_max_mtime(&filter_opts, vec![target]).expect("Nothing matched the target glob!");

    if base.modified_time() > target.modified_time() {
        exit(0)
    } else {
        exit(1)
    }
}

pub fn multi_check(filter_opts: FilterOpts, bases: Vec<Pattern>, targets: Vec<Pattern>) {
    let base = get_max_mtime(&filter_opts, bases).expect("Nothing matched the base globs!");

    let target = get_max_mtime(&filter_opts, targets).expect("Nothing matched the target globs!");

    if base.modified_time() > target.modified_time() {
        exit(0)
    } else {
        exit(1)
    }
}
