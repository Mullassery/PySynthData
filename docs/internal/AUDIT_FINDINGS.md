# Audit findings — 2026-10-19

This file records the repository-specific findings from the portfolio audit
conducted on 2026-10-19. It is documentation only; no code or release artifacts
were changed in this pass unless explicitly noted under "Implementation status".

Sources: the repository's own README/source/workflows, GitHub metadata and
Actions runs, PyPI metadata, and official competitor documentation. Claims are
marked as verified, inferred, or open questions. "Absent" means not found in the
inspected repository/docs, not proof that it cannot exist.

## What it is

Schema-driven synthetic relational data with foreign keys, constraints, differential privacy, streaming, quality scoring, and exports.

## Verified strengths

- Schema/YAML; FK ordering; constraints; Laplace DP budget; streaming; quality scoring; Pandas/Parquet/JSON; schema inference; 10 test files.
- Relational integrity + DP primitives are useful.

## Verified gaps and user-facing risks

- No ML-based synthesis; macOS arm64 wheel; unexposed Rust components; alpha.
- No fidelity/privacy benchmark vs SDV.

## Direct competitors

- SDV (ML synthesis, multi-table, evaluation, 3.5k stars), Faker, synthcity, Gretel/Tonic.

## Ranked next actions

- P1: publish Linux/Windows wheels (S/M).
- P2: expose implemented Rust components (S).
- P3: fidelity/privacy benchmark vs SDV (M).

## Do not build

- SDV/CTGAN parity; expensive and not the wedge.

## Evidence gaps / validation needed

- No benchmark; no DP guarantee validation.

## Recommended disposition

Maintain as a constrained relational generator; do not chase SDV.

## Implementation status

No code changes in this pass.
