# Audit Report: pysynthdata
Date: 2026-10-26 | Type: Read-only systems design audit  
Repo: https://github.com/Mullassery/PySynthData.git (local: /Users/georgimullassery/Desktop/Projects/pysynthdata) | Branch: main | Status: clean

## Overview
Rust-native library for generating synthetic relational datasets from schema. Foreign keys, constraints, differential privacy (Laplace), quality scoring, streaming, exports (Pandas/Parquet/JSON), schema inference from Postgres/MySQL/SQLite.

## Architecture / Entry points
- Rust core + Python bindings via PyO3/maturin (module `pysynthdata._core`, python-source `python/`). CLI `pysynthdata`. Workspace crates likely.

## Dependencies
Python: pandas, pyarrow. Optional: faker, pyyaml (dev), sqlalchemy (db). Maturin build.

## Data / State
- Relational generation with FK ordering, constraint enforcement, uniqueness. Streaming for large datasets. DP with ε-δ budget tracking.

## APIs / Contracts
- Python API + CLI. Schema YAML or Python API.

## Security observations
- Differential privacy implementation is a key security/privacy feature — needs verification of budget enforcement (prevents overspending per README).
- Schema inference connects to databases (Postgres/MySQL/SQLite) via SQLAlchemy — connection strings/secrets must not be logged.
- Generates synthetic data (privacy-safe by design) but DP parameters must be validated.
- No obvious hardcoded secrets.

## Tests / CI
- CI: ci.yml, commit-conventions.yml. Tests 13 found.

## Findings
| ID | Severity | Category | Evidence | Impact | Root cause | Recommendation | Confidence |
|---|---|---|---|---|---|---|---|
| PSD-001 | High | Privacy / Differential Privacy | README states "real ε-δ budget that prevents overspending" (Laplace mechanism). | If budget not actually enforced or composition incorrect, privacy guarantees fail. | Critical privacy feature. | Verify budget tracking is atomic, prevents reuse across generations, handles composition, documents privacy model; add tests for budget exhaustion. | High |
| PSD-002 | Medium | Security / DB Connections | Schema inference supports Postgres/MySQL/SQLite via SQLAlchemy. | Connection strings with credentials could be logged/exposed. | Database connectivity. | Ensure credentials never logged, use secure connections, validate inputs, warn on insecure configs. | Medium |
| PSD-003 | Low | Technical Debt | TODO/FIXME ~8621 (large count). | Maintenance burden. | Large codebase. | Triage critical items, especially DP/security related. | Medium |

## Strengths
- Real DP implementation claimed with budget enforcement. Relational integrity focus. Rust core performance. Practical exports.

## Prioritized remediation
1. Verify DP budget enforcement (PSD-001) with tests.
2. Harden DB connection handling (PSD-002).
3. TODO triage (PSD-003).

## Limitations
Read-only audit; did not inspect Rust DP implementation or budget tracking code.
