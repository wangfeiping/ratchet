use git_version::git_version;
use target_info::Target;

/// Returns the current version of this build of Ratchet.
///
/// A plus-sign (`+`) is appended to the git commit if the tree is dirty.
///
/// ## Example
///
/// `Ratchet/v0.0.0-***`
pub const VERSION: &str = git_version!(
    args = ["--always", "--dirty=+", "--abbrev=7"],
    prefix = "Ratchet/v0.1.0-",
    fallback = "unknown"
);

/// Returns `VERSION`, but with platform information appended to the end.
///
/// ## Example
///
/// `Ratchet/v0.0.0-***/x86_64-linux`
pub fn version_with_platform() -> String {
    format!("{}/{}-{}", VERSION, Target::arch(), Target::os())
}
