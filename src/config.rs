/// Represents how to treat symlinks.
#[derive(Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum SymlinkOption {
    /// Completely ignores any symlink.
    Omit,

    /// Includes symlinks themselves, but does not traverse them to retrieve metadata about the
    /// linked object.
    Include,

    /// Traverses symlinks and retrieves metadata about the linked object.
    Traverse,
}

/// Represents the glob filtering behavior specified by the user.
#[derive(Clone, Copy, Debug)]
pub struct FilterOpts {
    pub debug_mode: bool,
    pub include_files: bool,
    pub include_dirs: bool,
    pub symlinks: SymlinkOption,
}

/// By default, the script considers only file objects and traverses symlinks.
impl Default for FilterOpts {
    fn default() -> Self {
        Self {
            debug_mode: false,
            include_files: true,
            include_dirs: false,
            symlinks: SymlinkOption::Traverse,
        }
    }
}
