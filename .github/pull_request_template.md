## What this changes and why

## Checklist

- [ ] `cargo test --verbose` passes
- [ ] `cargo clippy --all-targets -- -D warnings` passes (zero warnings)
- [ ] `cargo fmt -- --check` passes
- [ ] `pytest python/tests` passes (if you touched the Python layer or the
      PyO3 boundary)
- [ ] If this adds a feature that returns a number/status: it's computed
      from real logic, not a hardcoded/plausible-looking placeholder. If the
      underlying capability isn't implemented, it returns an explicit
      `not_implemented` status with a reason instead.
- [ ] If this exposes a Rust module to Python for the first time, updated
      `README.md`'s Roadmap section and `../docs/internal/ROADMAP_HONEST.md` to reflect it's
      no longer Rust-only.
- [ ] Updated `CHANGELOG.md`'s `[Unreleased]` section.
