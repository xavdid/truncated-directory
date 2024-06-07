@_default:
  just --list

@build:
  cargo build --release
  cp target/release/truncated-directory ~/bin

@lint:
  cargo clippy

@lint-warnings:
  cargo clippy -- --deny warnings

@lint-fix:
  cargo clippy --fix --allow-dirty
