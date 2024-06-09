use std::{
    path::{Path, PathBuf},
    process::Command,
};

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

/// Removes a number of path parts from the front of a path.
/// functionally splits on `/`, so a leading slash will result in empty segments
pub fn shift_path_segments(path: &Path, skip_n: usize) -> String {
    path.components()
        .skip(skip_n)
        // .filter(|c| !matches!(c, Component::RootDir))
        .filter_map(|comp| comp.as_os_str().to_str().map(String::from))
        .collect::<Vec<String>>()
        .join("/")
        // if this is called with a leading slash,
        // then the root dir is treated as a compoent and joined with the resulting path.
        // so, clean that up
        .replace("//", "/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_git_repo() {
        // this test is always run from a git repo, so this will always return true
        // it's not a great test, but at least we'll know something is popping out

        let result = get_git_repo();
        assert!(result.is_some());

        assert!(result
            .unwrap()
            .to_str()
            .unwrap()
            .ends_with("truncated-directory"));
    }

    #[test]
    fn shifts_path_segments() {
        let path = shift_path_segments(Path::new("a/b/c"), 1);
        assert_eq!(path, String::from("b/c"));
    }

    #[test]
    fn counts_home_as_a_segment() {
        let path = shift_path_segments(Path::new("~/a/b/c"), 1);
        assert_eq!(path, String::from("a/b/c"));
    }

    #[test]
    fn counts_leading_slashes() {
        // None of the callsites pass a leading slash (at most, it's a leading `~`), but I want the behavior documented
        let path = shift_path_segments(Path::new("/a/b/c"), 2);
        assert_eq!(path, String::from("b/c"));
    }
}
