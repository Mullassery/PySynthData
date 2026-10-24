# Competitive intelligence in roadmap and upgrade work

**Policy date:** 2026-10-19

Competitive findings are an input to product decisions, not a one-off report.
Every pending roadmap item, new feature, and routine dependency upgrade should
check the repository's [`AUDIT_FINDINGS.md`](AUDIT_FINDINGS.md) before work
starts, and record the decision in the PR or changelog.

## Required check before building

1. **Read `AUDIT_FINDINGS.md`.** Find the relevant competitor row and the
   ranked next actions. If the finding is stale, update it with a dated source.
2. **Name the competitor and the user problem.** A feature is not justified by
   "competitor has it". State the workflow it unlocks, who needs it, and how
   often. Mark unverified demand as a hypothesis.
3. **Prefer reuse over new code.** Check whether an existing Mullassery repo
   already provides the capability (see `BUNDLES.md` in the profile repo and
   the bundle role file in this repo, when present). Typical reuse:
   - crawling/fetching/robots/SSRF → **MudFish**
   - architecture maps / PR diffs → **CodeAtlas**
   - data-quality contracts / validation → **StatGuardian**
   - token counting / cost tables → **PyTokenCalc**
   - dependency graph / advisories / SBOM → **PyDeepMesh**
   - segmentation / CLV / churn → **ClusterAudienceKit**
   - activation / lineage / PII → **PyReverseETL**
   - fleet state sync → **SyncFleet** (only after conflict resolution)
4. **Choose one disposition:** build, integrate, defer, or reject. Write it
   down. A rejection with a reason is a successful outcome.
5. **For routine upgrades**, check whether the upgrade closes a known
   competitor gap or breaks a differentiated capability. Run the tests that
   cover the affected path; do not merge a green build that silently changes a
   user-visible contract.

## PR / changelog checklist

- [ ] Competitor and source URL/date recorded (or "no direct competitor").
- [ ] User impact and frequency stated; unverified demand labelled.
- [ ] Reuse check done against sibling repos/bundles.
- [ ] Failure mode and rollback considered (especially auth, PII, cost).
- [ ] Docs/roadmap status updated honestly.
- [ ] Tests or a manual verification command included.

## Review cadence

- Refresh the competitor row when a release changes the capability.
- Re-check the full `AUDIT_FINDINGS.md` at least quarterly and before a major
  release.
- If a finding is wrong or stale, correct it in the same PR that relies on it.

## Non-goals

- Copying features for parity.
- Adding a dependency just because a competitor uses it.
- Treating star counts or download counts as proof of user need.

## OSS gap analysis — 2026-10-24

**What matters most here:** learned synthesis and statistical fidelity. Constraint sampling cannot reproduce joint distributions, and a constraint-violation score is not distribution fidelity; add copula/GAN/VAE synthesis and SDV-comparable quality metrics.

Scope: PySynthData 0.5.3 (Apache-2.0, alpha). Rust/PyO3 engine that generates relational data from YAML/Python schemas with foreign keys, constraints, Laplace-mechanism differential privacy with a sequential budget, streaming, constraint-based quality scoring, and pandas/CSV/Parquet/JSON export. Schema inference from SQLite/Postgres/MySQL via SQLAlchemy; optional Faker fields. Compared against the libraries a user would pick for synthetic data. Evidence: `README.md`, `AUDIT_FINDINGS.md`, `ROADMAP_HONEST.md`, `CHANGELOG.md`, `Cargo.toml`, `pyproject.toml`, `python/pysynthdata/`, and verified upstream pages. No fidelity or DP benchmark is claimed here because none exists in the repo.

### Comparable OSS projects
| Project | GitHub | License | Last activity | Why it is the comparison |
| --- | --- | --- | --- | --- |
| SDV | https://github.com/sdv-dev/SDV | Business Source License 1.1 (non-production use; change licence MIT) | release `v2.0.0`, 2026-10-15 | The reference multi-table synthesizer: learns statistical/ML models from real data, with quality/privacy evaluation |
| Faker | https://github.com/joke2k/faker | MIT | release `v40.43.0`, 2026-10-09 | The default choice for locale-aware fake values; PySynthData integrates it optionally |
| Synthcity | https://github.com/vanderschaarlab/synthcity | Apache-2.0 | release `v0.2.12`, 2025-05-08 | Research library of tabular/time-series/survival generators with privacy metrics and DP plugins |
| ydata-synthetic (now fg-data-synthetic) | https://github.com/Data-Centric-AI-Community/fg-data-synthetic | MIT | release `2.1.0`, 2026-04-23 | GAN/diffusion-style tabular and time-series synthesizers with a permissive licence |

### Capability comparison
| Capability | Ours | SDV | Faker | Synthcity | Gap |
| --- | --- | --- | --- | --- | --- |
| Schema-driven relational generation with FKs | Yes | Yes | No | No | Ours populates FKs from existing parents in dependency order; SDV does multi-table from learned models |
| Learns distributions from real data | No | Yes | No | Yes | Ours samples from declared types/constraints, not from a fitted model |
| Constraint enforcement (range, unique, enum, pattern) | Yes | Partial | No | No | Ours has explicit constraints; SDV validates metadata rather than arbitrary constraints |
| Formal differential privacy | Yes | Partial | No | Yes | Ours implements numeric Laplace with a sequential budget; SDV DP is an enterprise add-on; Synthcity ships DP-GAN/PATE/PrivBayes |
| Locale-aware realistic values | Partial | No | Yes | No | Ours gets locale behaviour only through optional Faker fields |
| Statistical fidelity and privacy evaluation | Partial | Yes | No | Yes | Ours scores constraint violations, not distribution distance or privacy risk |
| Streaming/chunked large generation | Yes | Partial | Partial | No | Ours streams entity chunks and Parquet; SDV samples batches |
| Export formats (pandas/CSV/Parquet/JSON) | Yes | Yes | Yes | Yes | Ours covers the common formats; no Iceberg/Delta writers |
| Schema inference from a live database | Yes | Partial | No | No | Ours infers via SQLAlchemy; only SQLite is end-to-end tested |
| Permissive licence for commercial production | Yes | No | Yes | Yes | SDV's BSL restricts production use of the licensed work |

### Gaps that matter (ranked)
1. **No learned synthesis (absent, partly deliberate).** The generator samples from declarative field types and constraints; it never fits marginals or correlations from a real sample. SDV and Synthcity learn Gaussian copulas, GANs, VAEs or TVAEs, which is what produces realistic joint distributions. `ROADMAP_HONEST.md` and `AUDIT_FINDINGS.md` reject full SDV/CTGAN parity, but there is no lightweight middle ground. Hurts any team that needs statistical realism rather than schema-valid fixtures. Effort: L for parity, S-M for a sample-fitted marginal/copula mode.
2. **Fidelity scoring is not statistical (weaker).** `World.fidelity_score` is computed from actual constraint violations, and the README is honest about that, but users compare it with SDV/Synthcity quality reports that measure column-shape, pairwise correlation and privacy distance. The number can be misread as distribution fidelity. Hurts trust and any "is this good enough?" decision. Effort: M.
3. **Differential privacy is narrow (weaker).** Only numeric fields receive Laplace noise, budget accounting is basic sequential composition, and there is no categorical DP, Rényi accountant, or per-column sensitivity model. SDV sells DP synthesizers and Synthcity exposes several DP mechanisms. Hurts privacy-sensitive teams who need a defensible guarantee beyond a demo. Effort: M.
4. **No benchmark against SDV (absent evidence).** `AUDIT_FINDINGS.md` lists "no fidelity/privacy benchmark vs SDV" and `ROADMAP_HONEST.md` leaves it open. The README says generation is deterministic and FKs are valid, but there is no measured quality, speed or privacy comparison. Hurts adoption against the category leader. Effort: M.
5. **Distribution and platform coverage (absent operationally).** The audit and README note only a macOS ARM64 wheel plus sdist; Linux/Windows users build from source. For a Rust extension that is a real install barrier. Hurts the first-run experience more than any missing feature. Effort: S-M.
6. **Schema inference is only verified on SQLite (weaker).** Postgres/MySQL share the SQLAlchemy `inspect()` path but were not exercised against live servers per `ROADMAP_HONEST.md`. Hurts teams whose whole reason to use this is inferring an existing relational schema. Effort: S-M.

### Where we are differentiated
- **Relational correctness first:** foreign keys always resolve to real parent rows in dependency order, with nullability/uniqueness/range/length/pattern/enum constraints enforced, not just generated columns.
- **One package for schema, FKs, constraints, DP, streaming and export:** SDV covers learned synthesis but not this exact constrained-fixture workflow; Faker has no relational model.
- **Rust engine with bounded memory:** streaming chunks and Parquet streaming for large datasets, deterministic seeds for reproducible CI fixtures.
- **Permissive licence:** Apache-2.0 throughout, while SDV's BSL 1.1 limits production use and DP features are an enterprise add-on.
- **Composability with Faker:** optional locale-aware providers plugged in after the Rust generator preserves relationships, instead of choosing one or the other.

### Disposition
- **Build:** a sample-fitted correlation mode (marginals plus simple copula) behind the existing schema API; statistical fidelity and privacy-distance report; categorical DP and a better accountant; reproducible benchmark harness against SDV/Synthcity; Linux/Windows wheels.
- **Integrate/reuse:** Faker/Mimesis for locale providers; reuse SDV or Synthcity only as benchmark oracles in a separate test environment, not as runtime dependencies (licence and weight).
- **Defer/reject:** full CTGAN/VAE parity (reject as the wedge, per audit), Iceberg/Delta writers (defer), MCP tool servers and the unexposed robotics/research modules (defer or delete), production compliance certification claims (reject).

### Sources
- https://github.com/sdv-dev/SDV (accessed 2026-10-24) — LICENSE BSL 1.1, release `v2.0.0` (2026-10-15), multi-table synthesis and evaluation
- https://docs.sdv.dev/sdv/single-table-data/modeling/synthesizers (accessed 2026-10-24) — community synthesizers and enterprise DP synthesizers
- https://github.com/joke2k/faker (accessed 2026-10-24) — MIT, release `v40.43.0` (2026-10-09)
- https://github.com/vanderschaarlab/synthcity (accessed 2026-10-24) — Apache-2.0, release `v0.2.12` (2025-05-08), privacy metrics and DP plugins
- https://github.com/Data-Centric-AI-Community/fg-data-synthetic (accessed 2026-10-24; formerly `ydataai/ydata-synthetic`) — MIT, release `2.1.0` (2026-04-23)
- https://github.com/lk-geimfari/mimesis (accessed 2026-10-24) — MIT, release `22.2.0` (2026-09-23), locale fake-data alternative
- Repo files inspected: `README.md`, `AUDIT_FINDINGS.md`, `ROADMAP_HONEST.md`, `CHANGELOG.md`, `Cargo.toml`, `pyproject.toml`, `python/pysynthdata/`
