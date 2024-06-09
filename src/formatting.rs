use std::path::PathBuf;

use colored::Colorize;

use super::filesystem::shift_path_segments;

pub fn format_repo_string(cwd: &PathBuf, repo_root: PathBuf) -> String {
    let repo_name = repo_root
        .file_name()
        .expect("unable to extract directory name from repo_root");

    let purple_git_root = repo_name.to_str().unwrap().purple().bold();

    if repo_root == *cwd {
        return purple_git_root.to_string();
    }

    let full_subpath = cwd
        .strip_prefix(repo_root)
        .expect("unable to strip git root from cwd");

    let subpath = match full_subpath.components().count() {
        subpath_len if subpath_len > 2 => {
            format!("…/{}", shift_path_segments(full_subpath, subpath_len - 2))
        }
        _ => full_subpath.display().to_string(),
    };

    format!("{purple_git_root}/{}", subpath.cyan().bold())
}

pub fn format_non_repo_string(cwd: &PathBuf, home_dir: &PathBuf) -> String {
    if home_dir == cwd {
        return format!("{}", "🏠 ~".purple().bold());
    }

    // clean if we're under home
    let cwd = if cwd.starts_with(home_dir) {
        let mut cleaned = PathBuf::from("~");
        cleaned.push(
            cwd.strip_prefix(home_dir)
                .expect("unable to strip homedir prefix from cwd"),
        );
        cleaned
    } else {
        cwd.clone()
    };

    let num_parts = cwd.components().count();

    let slice_from = if num_parts > 3 { num_parts - 3 } else { 0 };

    let cleaned_path = shift_path_segments(&cwd, slice_from);

    format!("{}", cleaned_path.purple().bold())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    macro_rules! repo_tests {
      ($($name:ident: $value:expr,)+) => {
        $(
          #[test]
          fn $name() {
              let (cwd, expected) = $value;
              assert_eq!(
                format_repo_string(
                  &PathBuf::from(cwd),
                  PathBuf::from("/Users/xavdid/projects/cool-thing")
                ),
                expected
              );
            }
          )*
        };
    }

    repo_tests!(
        repo_root: (
            "/Users/xavdid/projects/cool-thing",
            "cool-thing".purple().bold().to_string()
        ),
        repo_sub_1: (
            "/Users/xavdid/projects/cool-thing/sub",
            format!("{}/{}", "cool-thing".purple().bold(), "sub".cyan().bold()),
        ),
        repo_sub_2: (
            "/Users/xavdid/projects/cool-thing/sub/again",
            format!("{}/{}", "cool-thing".purple().bold(), "sub/again".cyan().bold()),
        ),
        repo_sub_3: (
            "/Users/xavdid/projects/cool-thing/one/last/time",
            format!("{}/{}", "cool-thing".purple().bold(), "…/last/time".cyan().bold()),
        ),
        repo_sub_4: (
            "/Users/xavdid/projects/cool-thing/one/last/time/final",
            format!("{}/{}", "cool-thing".purple().bold(), "…/time/final".cyan().bold()),
        ),
    );

    macro_rules! non_repo_tests {
      ($($name:ident: $value:expr,)+) => {
        $(
          #[test]
          fn $name() {
              let (cwd, expected) = $value;
              assert_eq!(
                format_non_repo_string(
                  &PathBuf::from(cwd),
                  &PathBuf::from("/Users/xavdid")
                ),
                expected
              );
            }
          )*
        };
    }

    non_repo_tests!(
      non_repo_home: (
        "/Users/xavdid",
        "🏠 ~".purple().bold().to_string(),
      ),
      non_repo_users: (
        "/Users",
        "/Users".purple().bold().to_string(),
      ),
      non_repo_root: (
        "/",
        "/".purple().bold().to_string(),
      ),
      non_repo_desktop: (
        "/Users/xavdid/Desktop",
        "~/Desktop".purple().bold().to_string(),
      ),
      non_repo_len_2: (
        "/Users/xavdid/Desktop/a",
        "~/Desktop/a".purple().bold().to_string(),
      ),
      non_repo_len_3: (
        "/Users/xavdid/Desktop/a/b",
        "Desktop/a/b".purple().bold().to_string(),
      ),
      non_repo_len_4: (
        "/Users/xavdid/Desktop/a/b/c",
        "a/b/c".purple().bold().to_string(),
      ),
      non_repo_len_5: (
        "/Users/xavdid/Desktop/a/b/c/d/",
        "b/c/d".purple().bold().to_string(),
      ),
    );
}
