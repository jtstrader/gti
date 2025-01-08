//! # Git Temp Ignore (GTI)
//!
//! GTI is a wrapper around git that streamlines temporarily ignoring files changes when
//! running commands. It's particularly useful if a build script, outside of your control,
//! makes minor changes to a repository that does not need to be tracked.
//!
//! It is possible in vanilla git to ignore these changes, but can become cumbersome when
//! re-running scripts when relevant major changes have already been made but not yet
//! committed.

fn main() {}
