use std::{fs::Metadata, path::PathBuf, time::SystemTime};

use crate::config::{FilterOpts, SymlinkOption};

#[derive(Clone, Debug)]
pub struct FileInfo<'a> {
    path: PathBuf,
    metadata: Metadata,
    opts: &'a FilterOpts,
}

impl<'a> FileInfo<'a> {
    pub fn new(path: PathBuf, metadata: Metadata, opts: &'a FilterOpts) -> Self {
        Self {
            path,
            metadata,
            opts,
        }
    }

    pub fn is_relevant(&self) -> bool {
        let o = self.opts;
        let m = &self.metadata;

        (o.include_files && m.is_file())
            || (o.include_dirs && m.is_dir())
            || (o.symlinks == SymlinkOption::Include && m.is_symlink())
    }

    pub fn modified_time(&self) -> SystemTime {
        match self.metadata.modified() {
            Ok(t) => t,
            Err(e) => panic!(
                "Unable to read modification time for {:?}: {}",
                self.path, e
            ),
        }
    }
}
