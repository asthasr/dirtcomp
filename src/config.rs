#[derive(Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum SymlinkOption {
    Omit,
    Include,
    Traverse,
}

#[derive(Clone, Copy, Debug)]
pub struct FilterOpts {
    pub include_files: bool,
    pub include_dirs: bool,
    pub symlinks: SymlinkOption,
}

impl Default for FilterOpts {
    fn default() -> Self {
        Self {
            include_files: true,
            include_dirs: false,
            symlinks: SymlinkOption::Traverse,
        }
    }
}
