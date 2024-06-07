use std::{
    path::{Path, PathBuf},
    process::Command,
};

#[must_use]
pub fn make_purple(s: &str) -> String {
    format!("%F{{13}}{s}%f")
}

#[must_use]
pub fn get_git_repo() -> Option<PathBuf> {
    let repo_root_call = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()
        .expect("something went wrong");

    if repo_root_call.status.success() {
        Some(PathBuf::from(
            String::from_utf8(repo_root_call.stdout)
                .expect("unable to convert git repo root to string")
                .trim(),
        ))
    } else {
        None
    }
}

/// Removes path parts from the front of a path.
///
/// ```
/// # use std::path::Path;
/// let path = shift_path_segments(Path::new("a/b/c"), 1);
/// assert_eq!(path, String::from("bb/c"));
/// ```
#[must_use]
pub fn shift_path_segments(path: &Path, skip_n: usize) -> String {
    path.components()
        .skip(skip_n)
        .filter_map(|comp| comp.as_os_str().to_str().map(String::from))
        .collect::<Vec<String>>()
        .join("/")
}
