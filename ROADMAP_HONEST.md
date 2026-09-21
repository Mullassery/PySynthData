# Roadmap (honest)

This file states plainly what works, what's broken, what's missing, and what
technical debt exists. No hedge language for things that are simply not
built. See `README.md` for the user-facing feature description and
`CHANGELOG.md` for what changed in the most recent hygiene pass.

## 1. Works today (tested by me this session)

- **Core generation engine** (`src/generator.rs`, `src/schema.rs`,
  `src/parser.rs`): schema definition, typed field generation, FK
  population, constraint enforcement, quality scoring. Verified with
  `cargo test --lib` — **70/70 passed** — and the full integration suite in
  `tests/*.rs` (behaviors, monitoring, parser, research, robotics,
  ros2_bridge, schema — **55 additional tests, all passed**, run via
  `cargo test --verbose`).
- **Differential privacy** (`src/privacy.rs`, `PrivacyBudget`,
  `generate_private()`): real Laplace-mechanism noise, real epsilon-delta
  sequential-composition budget accounting that errors on overspend.
  Verified via `python/tests/test_privacy_and_streaming.py` (9 tests,
  passed).
- **Streaming/chunked generation** (`generate_streaming()`,
  `to_parquet_streaming()`): verified via the same pytest file (5 tests,
  passed) — chunk boundaries, FK correctness across chunks, and error
  propagation from the worker thread were all exercised, not just the happy
  path.
- **Live DB schema inference** (`python/pysynthdata/db_schema.py`,
  `pysynthdata infer-schema` CLI command): verified via
  `python/tests/test_db_schema.py` (9 tests, passed) against a real SQLite
  database (in-memory/tempfile) exercising single-table and multi-table +
  FK-relationship inference, the YAML round-trip, and CLI error handling for
  a missing table. Postgres/MySQL code paths share the same SQLAlchemy
  `inspect()` call and were **not separately exercised against a live
  Postgres/MySQL server in this sandbox** (no network access) — this is an
  environment limitation, not a known bug, but it means only the SQLite path
  has been end-to-end verified against a real database engine.
- **Rust toolchain checks**: `cargo fmt -- --check` clean, `cargo clippy
  --all-targets -- -D warnings` clean (zero warnings under
  `-D warnings`).
- **`.github/workflows/ci.yml`**: passes `actionlint` with zero findings.
  Uses current, non-deprecated actions (`actions/checkout@v4`,
  `dtolnay/rust-toolchain@stable`, `PyO3/maturin-action@v1`,
  `actions/upload-artifact@v4`). No `|| true`/`|| echo` failure-masking
  found anywhere in the workflow.

## 2. Exists in Rust but has no Python binding (verified still true)

`src/robotics.rs`, `src/ros2_bridge.rs`, `src/behaviors.rs`, `src/research.rs`,
`src/monitoring.rs`, `src/real_world_mess.rs`, `src/unconventional_data.rs`,
`src/multimodal_augmentation.rs`. Each has real Rust-level test coverage
(part of the 125 total Rust tests above) but none of these types are
registered in the PyO3 module (`src/lib.rs`) or exposed in
`python/pysynthdata/api.py`. Not usable from `import pysynthdata`. This was
already accurately disclosed in `README.md` prior to this pass; confirmed
still accurate.

## 3. Explicitly not implemented (returns `not_implemented`, not a fake number)

`check_privacy_compliance`, `detect_pii_exposure`, `anonymize_dataset`,
`check_fairness_bias`, `debias_dataset`, `validate_distribution_match`,
`augment_minority_class`, `evaluate_synthetic_utility` in
`pysynthdata/_mcp_tools.py`. See section 5 below — this file is currently
dead code with no server loading it, so these honest `not_implemented`
returns are not reachable from anything today either.

No Iceberg/Delta Lake writers exist — only Parquet/JSON/CSV/pandas export.

No Rényi DP accountant — only basic sequential composition for
`PrivacyBudget`. A real but conservative choice, not a bug.

## 4. Bugs / gaps fixed in this pass

- **Version drift**: `python/pysynthdata/__init__.py` had
  `__version__ = "0.4.0"` while `Cargo.toml` and `pyproject.toml` were both
  at `0.5.0`, and the installed dev-venv wheel reported `0.4.0` at runtime
  even after a fresh `maturin develop --release` rebuild (confirmed by
  running `python -c "import pysynthdata; print(pysynthdata.__version__)"`
  before and after the fix). Fixed to `"0.5.0"`.
- **Cross-project boilerplate contamination**: a top-level `pysynthdata/`
  directory (separate from the real, packaged `python/pysynthdata/`)
  contained `_mcp_connector.py` (imports a nonexistent `statguardian`
  package, shells out to a nonexistent `dab` binary), `cli_dashboard.py`
  (printed fabricated metrics — "2.3M total", "185K records/min", "97.2%
  quality" — as if they were real generator output), `cli.py` (a shadow CLI
  entry point that only wired up the fake dashboard), `scripts/
  post_install.py` (advertised `pysynthdata generate`, `pysynthdata
  dashboard`, `pysynthdata export` — none of which exist in the real CLI),
  `scripts/setup_shortcuts.sh` (shell aliases for the fake dashboard), and
  `scripts/test_dashboards_crossplatform.sh` (a "test" script that checks
  for `pystreamai`, `pystreammcp`, `pystreampdf`, `pystreamxl`,
  `statguardian`, `pyreverseetl`, `pyterrainmap`, `pyroboreplay`,
  `pyrobosimulator` — nine packages that have nothing to do with
  PySynthData). All deleted. None of it shipped in the PyPI wheel/sdist
  (confirmed: `pyproject.toml`'s `python-source = "python"` only packages
  `python/pysynthdata/`), so this was dead-in-production but very much live
  in the repo tree and actively misleading to a reader.
- **Dead default binary**: `src/main.rs` was an untouched `cargo new`
  "Hello, world!" stub, unrelated to the real `cli` binary in
  `src/bin/cli.rs`. Deleted; confirmed `cargo build`/`cargo test` still pass
  and nothing referenced `cargo run`.
- **CI never ran the Python test suite**: `ci.yml` had `test` (cargo test),
  `lint` (fmt/clippy), and `build-wheels` (maturin build only, no install,
  no test) jobs — `python/tests/` (14 tests) never ran in CI. Added a
  `python-tests` job.

## 5. Technical debt — not fixed in this pass, needs a dedicated follow-up

1. **Compiled binary committed to git**:
   `python/pysynthdata/_core.abi3.so` is tracked in git despite `*.so` being
   in `.gitignore` (it was added before the ignore rule, or force-added, and
   every rebuild since has produced a binary diff in the commit history —
   confirmed via `git log --oneline -- python/pysynthdata/_core.abi3.so`,
   which shows the file changed in at least 4 separate feature commits). This
   is (a) an arm64-macOS-specific binary that won't run on other
   platforms/architectures, so it's not even universally useful as a
   checked-in artifact, (b) bloats repo history with binary diffs, and (c)
   is unauditable — nobody reviewing a PR diff can see what changed in a
   `.so` file. Fix: `git rm --cached python/pysynthdata/_core.abi3.so`,
   verify `.gitignore`'s existing `*.so` line actually excludes it going
   forward, and confirm the build/release process (`maturin build`/CI
   wheel-build job) doesn't depend on this file being pre-built and
   committed. Left undone here because removing a long-tracked binary from
   history/on-disk needs a decision about whether to rewrite history
   (out of scope for a hygiene pass) or just stop tracking it going forward.
2. **`pysynthdata/_mcp_tools.py` is orphaned**: after deleting
   `_mcp_connector.py` (the fake boilerplate that used to call into it),
   nothing in the repo starts an MCP server that loads
   `pysynthdata/_mcp_tools.py`. It's honestly written (real
   `not_implemented` returns, no fabricated numbers) but currently
   unreachable dead code — not part of the packaged wheel, not imported by
   any test, not wired to any entry point. Follow-up decision needed:
   either (a) build a real minimal MCP stdio/HTTP server entry point that
   actually loads it, or (b) delete it too and drop the MCP framing
   entirely. Left as-is (not deleted) in this pass because it's not
   misleading in isolation — the README now explicitly discloses it's
   dead code — and a asymmetric decision (build vs. delete) shouldn't be
   made silently in a documentation-first pass.
3. **Postgres/MySQL `infer-schema` paths are untested against real
   servers in this environment**: only the SQLite path has an end-to-end
   test with a real database engine (see section 1). The Postgres/MySQL
   code shares the same SQLAlchemy `inspect()`-based implementation, so the
   risk is likely low, but it is genuinely unverified here, not "tested and
   just happens to also work for Postgres/MySQL."
4. **No Python lint/type-check job in CI**: `README.md`'s Development
   section recommends `ruff check python/ pysynthdata/` and `pyproject.toml`
   configures `mypy` (`strict = true`) and `black`, but none of the three
   run in `ci.yml` — only `pytest` was added in this pass. A regression in
   Python-side style/typing would not be caught by CI today. Ran `ruff
   check python/ pysynthdata/` manually in this pass: **19 E501
   (line-too-long, >100 chars) findings**, all cosmetic docstring/test
   lines, no logic issues — spread across `pysynthdata/_mcp_tools.py` (15),
   `python/pysynthdata/api.py` (2), `python/pysynthdata/scripts/
   post_install.py` (1), `python/tests/test_privacy_and_streaming.py` (1).
   Not fixed here (19 scattered line-wraps is more than a "small safe" edit
   and risks introducing docstring-formatting churn); left for whoever adds
   the CI lint job to fix alongside it.
8. **`python/pysynthdata/scripts/post_install.py` is never actually
   invoked**: it ships inside the packaged wheel (unlike the deleted
   top-level `pysynthdata/scripts/post_install.py`, this one is honest —
   correct commands, no fabricated metrics) but nothing calls
   `post_install()` — there's no `setup.py` post-install hook or
   entry-point wired to it, and `pip install` does not run arbitrary
   scripts inside installed packages. It is dead code that happens to be
   truthful, as opposed to the deleted copy, which was dead code that lied.
   Low priority: either wire it to something (e.g., print it from
   `pysynthdata.cli:main` on first run) or delete it.
5. **No dependency/vulnerability scanning has ever actually run against
   this dependency tree**: a `dependency-audit` CI job (`cargo audit`) was
   added in this pass, but it has not been executed even once — this
   sandbox has no network access to crates.io. The Rust dependency tree
   (`Cargo.lock`, 23KB / dozens of transitive deps including `chrono`,
   `regex`, `uuid`, `rayon`) has not been checked for known CVEs by anyone,
   as far as this audit could determine. Treat the new CI job as "wired up,
   unverified" until it runs once for real on GitHub Actions.
6. **`build-wheels` CI job builds but never smoke-tests the wheel**: it
   uploads an artifact but doesn't `pip install` it and run anything against
   it, so a wheel that builds successfully but is broken at import time
   (e.g., a packaging/`include` misconfiguration) wouldn't be caught before
   a maintainer manually tests it pre-release.
7. **No `docs/architecture/` directory was added.** Judged not warranted:
   the project is one Rust crate with a thin PyO3 wrapper, and `README.md`'s
   existing "What's real here" / "What's intentionally not here" sections
   already describe the module boundaries and data flow at the level of
   detail a reader needs. Adding Mermaid diagrams for a project this size
   would be documentation for its own sake, not because the architecture is
   hard to follow from the README + source tree.

## 6. Not evaluated in this pass (explicitly out of scope / environment limits)

- **No live network access** in this sandbox: could not run `cargo audit`,
  `pip-audit`, `cargo outdated`, or hit github.com/crates.io/pypi.org to
  check for newer dependency versions or actually-published CVEs. Every
  claim above about "this hasn't been checked for CVEs" is a statement about
  what has and hasn't been verified, not a claim that vulnerabilities exist.
- **Did not test Windows** (`ci.yml`'s `test` job matrix includes
  `windows-latest`, but this session ran entirely on macOS arm64).
