# truncated-directory

This is a little Rust CLI use to display the path in my [starship](https://starship.rs/) shell prompt on macOS and Linux. It's based on [this issue](https://github.com/starship/starship/issues/4199).

## Installation

It's not available in a package manager, so just clone and compile it on your own. You can point starship it, wherever it lives. See [how I do it](https://github.com/xavdid/dotfiles/blob/3a913be5f8a93bfce510ff80563c22c4feec516e/config/starship.toml#L63-L67) if you want an example of usage.

## Examples

Here's how each of the following directories is displayed:

```
/                              <-- /
└── Users/                     <-- /Users
    └── xavdid/                <-- 🏠 ~
        ├── projects/          <-- ~/projects
        │   └── some-repo/     <-- some-repo
        │       └── a/         <-- some-repo/a
        │           └── b/     <-- some-repo/a/b
        │               └── c/ <-- some-repo/…/b/c
        └── Desktop/           <-- ~/Desktop
            └── d/             <-- ~/Desktop/d
                └── e/         <-- Desktop/d/e
                    └── f/     <-- d/e/f
```

## Logic

It prints the current directory based on the following logic:

- if not under a git repo:
  - the current directory and (at most) the two above it.
  - replaces home with `~` when visible
- if under a git repo:
  - always show the directory of the repo root
  - show at most 2 directories under that
  - if you're more than two directories down from the repo root, truncate everything between
