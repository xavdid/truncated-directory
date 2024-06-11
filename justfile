@_default:
  just --list

# compile for release and move to a directory
@build dest="~/bin/":
  cargo build --release
  cp target/release/truncated-directory {{dest}}

@lint:
  cargo clippy

# lint, but treat warnings as errors
@lint-warnings:
  cargo clippy -- --deny warnings

@lint-fix:
  cargo clippy --fix --allow-dirty
