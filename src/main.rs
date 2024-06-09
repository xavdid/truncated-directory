use std::{env::current_dir, path::PathBuf};

use home::home_dir as get_home_dir;

mod filesystem;
mod formatting;

use formatting::{format_non_repo_string, format_repo_string};

fn get_truncated_path(
    cwd: &PathBuf,
    home_dir: &PathBuf,
    maybe_repo_root: Option<PathBuf>,
) -> String {
    match maybe_repo_root {
        Some(git_root) => format_repo_string(cwd, git_root),
        None => format_non_repo_string(cwd, home_dir),
    }
}

fn main() {
    print!(
        "{}",
        get_truncated_path(
            &current_dir().expect("err fetching cwd"),
            &get_home_dir().expect("unable to get home dir"),
            filesystem::get_git_repo()
        )
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Colorize;
    #[cfg(test)]
    use pretty_assertions::assert_eq;

    macro_rules! root_tests {
        ($($name:ident: $value:expr,)+) => {
          $(
            #[test]
            fn $name() {
                let (cwd, maybe_repo, expected) = $value;
                assert_eq!(
                  get_truncated_path(
                    &PathBuf::from(cwd),
                    &PathBuf::from("/Users/xavdid"), // home
                    maybe_repo
                  ),
                  expected
                );
              }
            )*
          };
      }

    root_tests!(
        root_home: (
            "/Users/xavdid",
            None,
            "🏠 ~".purple().bold().to_string()
        ),
        root_users: (
            "/Users",
            None,
            "/Users".purple().bold().to_string()
        ),
        root_root: (
            "/",
            None,
            "/".purple().bold().to_string()
        ),
        root_desktop: (
            "/Users/xavdid/Desktop",
            None,
            "~/Desktop".purple().bold().to_string()
        ),
        root_repo_root: (
            "/Users/xavdid/projects/cool-thing",
            Some(PathBuf::from("/Users/xavdid/projects/cool-thing")),
            "cool-thing".purple().bold().to_string()
        ),
        root_repo_sub: (
            "/Users/xavdid/projects/cool-thing/sub",
            Some(PathBuf::from("/Users/xavdid/projects/cool-thing")),
            format!("{}/{}", "cool-thing".purple().bold(), "sub".cyan().bold()),
        ),
        root_repo_deep_sub: (
            "/Users/xavdid/projects/cool-thing/one/last/time/final",
            Some(PathBuf::from("/Users/xavdid/projects/cool-thing")),
            format!("{}/{}", "cool-thing".purple().bold(), "…/time/final".cyan().bold()),
        ),
        root_deep_len: (
            "/Users/xavdid/Desktop/a/b/c/d/",
            None,
            "b/c/d".purple().bold().to_string(),
        ),
    );
}
