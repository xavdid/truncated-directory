use std::{env::current_dir, path::PathBuf};

use home::home_dir as get_home_dir;

use truncated_directory::filesystem::{get_git_repo, shift_path_segments};
// use truncated_directory::{make_bold_cyan, make_purple, print_purple};

#[must_use] pub fn make_purple(s: &str) -> String {
    format!("%F{{13}}{s}%f")
}

#[must_use] pub fn make_bold_cyan(s: &str) -> String {
    format!("%B%F{{14}}/{s}%f%b")
}

pub fn print_purple(s: &str) {
    print!("{}", make_purple(s));
}

fn main() {
    let cwd = current_dir().expect("err fetching cwd");

    if let Some(git_root) = get_git_repo() {
        let repo_name = git_root
            .file_name()
            .expect("unable to extract directory name from git_root");

        let purple_git_root = make_purple(repo_name.to_str().unwrap());

        if git_root == cwd {
            print!("{purple_git_root}");
            return;
        }

        let sub_path = cwd
            .strip_prefix(git_root)
            .expect("unable to strip git root from cwd");

        let sub_path_components = sub_path.components().count();

        if sub_path_components <= 2 {
            let cyan_sub_path = make_bold_cyan(&sub_path.display().to_string());
            print!("{purple_git_root}{cyan_sub_path}");
            return;
        }

        let truncated_path = shift_path_segments(sub_path, sub_path_components - 2);

        let cyan_truncated_path = make_bold_cyan(&format!("…/{truncated_path}"));

        print!("{purple_git_root}{cyan_truncated_path}");
    } else {
        let home_dir = get_home_dir().expect("unable to get home dir");

        if home_dir == cwd {
            print_purple("🏠 ~");
            return;
        }

        // clean if we're under home
        let cwd = if cwd.starts_with(&home_dir) {
            let mut cleaned = PathBuf::from("~");
            cleaned.push(
                cwd.strip_prefix(&home_dir)
                    .expect("unable to strip homedir prefix from cwd"),
            );
            cleaned
        } else {
            cwd
        };

        let num_parts = cwd.components().count();

        let slice_from = if num_parts > 3 { num_parts - 3 } else { 0 };

        let cleaned_path = shift_path_segments(&cwd, slice_from);

        print_purple(&cleaned_path);
    }
}
