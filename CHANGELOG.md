# Changelog

All notable changes to this project are documented here. Format loosely
follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/). No
fabricated history — entries below `[Unreleased]` only exist once they are
part of a tagged release; this file was added during an OSS-hygiene pass and
does not retroactively reconstruct every prior release's notes (see `git log`
for the real commit history).

## [Unreleased]

### Removed
- Deleted a top-level `pysynthdata/` directory tree (`cli.py`,
  `cli_dashboard.py`, `_mcp_connector.py`, `scripts/post_install.py`,
  `scripts/setup_shortcuts.sh`, `scripts/test_dashboards_crossplatform.sh`)
  and the root `pysynthdata.toml`. This was leftover boilerplate copy-pasted
  from an unrelated sibling project: `_mcp_connector.py` imported a
  nonexistent `statguardian` package and shelled out to a `dab` binary that
  doesn't exist here; `cli_dashboard.py` printed fabricated metrics ("2.3M
  total", "185K records/min", "97.2%" quality) as if they were real
  generation output; `post_install.py` advertised `pysynthdata generate`,
  `pysynthdata dashboard`, and `pysynthdata export` subcommands that have
  never existed in the real CLI; `test_dashboards_crossplatform.sh` tested
  imports of `pystreamai`, `pystreammcp`, `pystreampdf`, `pystreamxl`,
  `statguardian`, `pyreverseetl`, `pyterrainmap`, `pyroboreplay`,
  `pyrobosimulator` — none of which are PySynthData. None of this was part
  of the built/published package (`pyproject.toml`'s `python-source =
  "python"` only ever packaged `python/pysynthdata/`), so nothing installed
  from PyPI was affected. `pysynthdata/_mcp_tools.py` (the one honest file
  in that tree — real tool schemas backed by the real generation engine) was
  kept, but is now explicitly documented in the README as dead code with no
  server that loads it.
- Deleted `src/main.rs`, an untouched `cargo new` "Hello, world!" stub that
  built a useless default binary target alongside the real `cli` binary
  (`src/bin/cli.rs`). Confirmed nothing (`README.md`, CI, docs) invoked
  `cargo run`.

### Fixed
- `python/pysynthdata/__init__.py` `__version__` was hardcoded to `"0.4.0"`
  while `Cargo.toml`/`pyproject.toml` were both at `0.5.0`. Bumped to
  `"0.5.0"` to match.

### Added
- `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`,
  `ROADMAP_HONEST.md`, `.github/dependabot.yml`,
  `.github/ISSUE_TEMPLATE/bug_report.yml`,
  `.github/ISSUE_TEMPLATE/feature_request.yml`,
  `.github/pull_request_template.md`.
- CI: a `python-tests` job that builds the extension with `maturin develop`
  and runs `pytest python/tests` (previously CI built wheels but never ran
  the Python test suite). A `dependency-audit` job running `cargo audit`
  (unverified — this sandbox has no network access to crates.io/PyPI, so it
  has not actually been executed against the current dependency tree).

See `ROADMAP_HONEST.md` for what is and isn't implemented, and for
technical-debt items intentionally left unfixed in this pass.
