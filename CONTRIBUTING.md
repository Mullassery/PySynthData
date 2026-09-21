# Contributing to PySynthData

## Setup

```bash
git clone https://github.com/Mullassery/PySynthData.git
cd PySynthData
pip install maturin
maturin develop --release      # builds the Rust extension into your active venv
pip install -e ".[dev,db]"
```

On macOS you may need to point PyO3 at a Python >= 3.10 explicitly if your
default `python3` is older (the crate declares `abi3-py310`):

```bash
export PYO3_PYTHON=/path/to/python3.11
```

If linking fails on macOS with undefined symbols, set:

```bash
export RUSTFLAGS="-C link-args=-undefined -C link-args=dynamic_lookup"
```

## Before opening a PR

Run all of these locally — CI runs the same checks and will fail the build
otherwise:

```bash
cargo test --verbose
cargo clippy --all-targets -- -D warnings
cargo fmt -- --check
pytest python/tests
ruff check python/ pysynthdata/
```

- `cargo fmt` and `cargo clippy -D warnings` are enforced in CI with zero
  tolerance — a single warning fails the build.
- Add Rust tests in `tests/` (integration) or `#[cfg(test)] mod tests` in the
  relevant `src/*.rs` file. Add Python tests in `python/tests/` for anything
  touching the PyO3 boundary (threading, pyarrow export, CLI argument
  parsing) that a Rust unit test can't reach.

## Honesty bar for this repo

This project has a history of having fabricated/compliance-theater code
removed (see `git log` for "compliance theater" and boilerplate-contamination
cleanups, and `CHANGELOG.md`/`ROADMAP_HONEST.md` for the most recent pass).
Concretely:

- Do not add a function that returns a hardcoded or plausible-looking number
  when the real computation isn't implemented. Return an explicit
  `not_implemented` status with a reason instead.
- Do not copy boilerplate from another repo without checking every reference
  (package names, example commands, printed messages) actually applies to
  *this* project.
- If you add a Rust module that isn't exposed through the Python API yet,
  say so in the README's Roadmap section rather than implying it's usable
  from `import pysynthdata`.

## Reporting bugs / requesting features

Use the GitHub issue templates (`.github/ISSUE_TEMPLATE/`). For security
issues, see `SECURITY.md` instead of opening a public issue.

## License

By contributing, you agree your contributions are licensed under the
project's [Apache License 2.0](LICENSE).
