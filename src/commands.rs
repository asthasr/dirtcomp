use std::process::exit;

use glob::Pattern;

use crate::{config::FilterOpts, traversal::get_max_mtime};

/// Represents the OS status code that will be returned. See the `exit` method for numeric values.
enum ReturnCodes {
    ComparisonTrue,
    ComparisonFalse,
    NoEntityError,
}

impl ReturnCodes {
    /// Exits with an appropriate status code for the command outcome.
    fn exit(&self) -> ! {
        match self {
            Self::ComparisonTrue => exit(0),
            Self::ComparisonFalse => exit(1),
            Self::NoEntityError => exit(2),
        }
    }
}

/// Allows the construction of an appropriate return code based on a single comparison.
impl From<bool> for ReturnCodes {
    fn from(value: bool) -> Self {
        if value {
            Self::ComparisonTrue
        } else {
            Self::ComparisonFalse
        }
    }
}

/// Checks one base glob against one target glob.
pub fn simple_check(filter_opts: FilterOpts, base: Pattern, target: Pattern) {
    multi_check(filter_opts, &[base], &[target]);
}

/// Checks a slice of base globs against a slice of target globs.
pub fn multi_check(filter_opts: FilterOpts, bases: &[Pattern], targets: &[Pattern]) {
    let Some((_, base_mtime)) = get_max_mtime(&filter_opts, bases) else {
        eprintln!("Nothing matched the base glob(s).");
        ReturnCodes::NoEntityError.exit()
    };

    let Some((_, target_mtime)) = get_max_mtime(&filter_opts, targets) else {
        eprintln!("Nothing matched the target glob(s).");
        ReturnCodes::NoEntityError.exit()
    };

    ReturnCodes::from(base_mtime > target_mtime).exit()
}
