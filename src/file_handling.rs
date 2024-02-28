use std::{fs::Metadata, path::PathBuf, time::SystemTime};

use anyhow::{Context, Result};

use crate::config::{FilterOpts, SymlinkOption};

/// Represents a path, the metadata of the filesystem object it represents, and the `dirtcomp`
/// options that affect its relevance to the comparisons.
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

    /// Determines whether the file represented by this structure is relevant to the comparison.
    pub fn is_relevant(&self) -> bool {
        let o = self.opts;
        let m = &self.metadata;

        (o.include_files && m.is_file())
            || (o.include_dirs && m.is_dir())
            || (o.symlinks == SymlinkOption::Include && m.is_symlink())
    }

    /// Retrieves the modified time for the filesystem object from the operating system. This is a
    /// fallible operation.
    pub fn modified_time(&self) -> Result<SystemTime> {
        self.metadata.modified().with_context(|| {
            format!(
                "Could not read modified time for {}",
                self.path.to_str().unwrap_or("(unknown path)")
            )
        })
    }
}
