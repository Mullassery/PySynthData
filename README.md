# PySynthData

## Problem

Testing and demoing data pipelines usually means either using real
(sensitive, hard-to-share) production data, or hand-writing fake rows that
don't respect the foreign-key/constraint structure a real schema has —
so joins don't reflect reality and quality checks have nothing real to
catch.

## Solution

**Generate synthetic relational datasets from a schema.**

Define entities, fields, foreign-key relationships, and constraints; get back real, typed rows — with foreign keys that actually point at rows that exist, constraints that are actually enforced, and a quality report that actually measures violations instead of returning a hardcoded number. Row generation and quality scoring happen in a compiled Rust core; the Python layer is a thin wrapper around it.

[![PyPI](https://img.shields.io/pypi/v/pysynthdata)](https://pypi.org/project/pysynthdata/)
[![Python 3.10+](https://img.shields.io/badge/Python-3.10%2B-blue)](https://www.python.org)
[![CI](https://github.com/Mullassery/PySynthData/actions/workflows/ci.yml/badge.svg)](https://github.com/Mullassery/PySynthData/actions/workflows/ci.yml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](./LICENSE)

---

## Use cases

- **Generating realistic test/demo data for a relational schema** without
  hand-writing fixtures — FK integrity and constraints are enforced, not
  approximated.
- **Reverse-engineering a schema from a live database** you don't want to
  hand-model — `infer-schema` reflects real Postgres/MySQL/SQLite
  structure via SQLAlchemy.
- **Privacy-safe synthetic data with a real epsilon-delta budget** —
  `generate_private()`, when the source data's actual values shouldn't
  leak even through a synthetic derivative.
- **Not yet a good fit for:** robotics/ROS2 simulation data, behavioral
  state-machine scenarios, or multimodal (vision/audio/sensor) data — all
  real in the Rust core but not yet exposed through the Python API (see
  [Roadmap](#roadmap)).

## Status

`0.5.0`, matches the version live on PyPI — no drift. The core pipeline — schema definition, row generation, foreign keys, constraints, quality scoring, pandas/Parquet/JSON export, differential-privacy noise injection, chunked/streaming generation, and live-database schema inference — is implemented and tested end to end. Several bundled Rust modules (robotics fleet simulation, ROS2 bridge, behavioral state machines, domain research knowledge base, monitoring/drift detection, "real world mess" injectors) exist in the codebase but are **not yet exposed through the Python API** — see [Roadmap](#roadmap).

## Install

```bash
pip install pysynthdata
```

## Quick start

```python
from pysynthdata import Schema, WorldGenerator

schema = Schema()
schema.add_entity("customers")
schema.add_field("customers", "id", "uuid", unique=True)
schema.add_field("customers", "name", "string")
schema.add_field("customers", "age", "int")
schema.add_field("customers", "status", "enum(active,suspended,closed)")
schema.add_constraint("range", "customers", "18-90", field="age")

schema.add_entity("orders")
schema.add_field("orders", "id", "uuid", unique=True)
schema.add_field("orders", "customer_id", "uuid")
schema.add_field("orders", "amount", "float")
schema.add_relationship("customers", "orders", "id", "customer_id", "1:n")

generator = WorldGenerator(schema)
world = generator.generate(num_records=1000, seed=42)

df = world.to_pandas("customers")          # real pandas DataFrame
world.to_parquet("out/")                   # one .parquet file per entity
world.to_json("out/world.json")            # all entities, one JSON file

print(world.fidelity_score)                # 1.0 = zero detected constraint violations
print(world.quality_report)                # {'fidelity_score':..., 'null_violations':..., ...}
```

Every `orders.customer_id` value in the output is drawn from an `id` that was actually generated for `customers` — foreign keys are real, not independently-random UUIDs. Generation is deterministic: the same schema + `seed` always produces the same rows.

### Loading a schema from YAML

```python
from pysynthdata import WorldGenerator

generator = WorldGenerator.from_yaml("examples/banking_schema.yaml")
world = generator.generate(num_records=5000, seed=7)
```

See [`examples/banking_schema.yaml`](examples/banking_schema.yaml) for the full YAML shape (entities, fields, relationships, constraints).

### Inferring a schema from a live database

```bash
pip install pysynthdata[db]

# One table
pysynthdata infer-schema --db-url "postgresql+psycopg://user:pass@host/dbname" \
    --table customers --output customers_schema.yaml

# Every table, including foreign-key relationships between them
pysynthdata infer-schema --db-url "mysql+pymysql://user:pass@host/dbname" \
    --all-tables --output full_schema.yaml
```

`infer-schema` connects to a real Postgres, MySQL, or SQLite database and reflects actual table structure via SQLAlchemy's `inspect()` API (`python/pysynthdata/db_schema.py`) — column types, nullability, primary keys, unique constraints, and foreign keys are read from the database itself, not guessed. The output is a YAML file in the same `entities`/`relationships` shape as [`examples/banking_schema.yaml`](examples/banking_schema.yaml), loadable directly with `Schema.from_yaml()` / `WorldGenerator.from_yaml()`. Postgres and MySQL go through the identical SQLAlchemy code path — only the `--db-url` scheme differs (`postgresql+psycopg://...` / `postgresql+psycopg2://...` for Postgres, `mysql+pymysql://...` for MySQL); each backend's driver (`psycopg`/`psycopg2`, `pymysql`) needs to be installed separately since SQLAlchemy doesn't bundle DB drivers itself. `--all-tables` also emits a `relationships` entry for every foreign key found between the inferred tables, with `cardinality` set to `1:1` when the FK column is itself unique or a primary key, `1:n` otherwise.

### Differential privacy and large-batch streaming export

```python
from pysynthdata import PrivacyBudget

# Add real Laplace-mechanism noise to every Int/Float field, tracked against
# a real epsilon-delta budget that errors rather than silently overspending.
budget = PrivacyBudget(epsilon=1.0, delta=1e-5)
private_world = generator.generate_private(num_records=1000, seed=42, budget=budget, epsilon=0.3)
print(private_world.privacy_report)   # {'epsilon_spent': 0.3, 'values_perturbed': ..., ...}
print(budget.remaining_epsilon)       # 0.7

# For batches too large to hold in memory at once: stream generation in
# chunks, or write straight to Parquet with peak memory bounded by chunk_size.
for entity_name, rows in generator.generate_streaming(num_records=5_000_000, seed=1, chunk_size=50_000):
    ...  # process one chunk at a time

generator.to_parquet_streaming(num_records=5_000_000, seed=1, out_dir="out/", chunk_size=50_000)
```

## What's real here

- **Row generation** (`src/generator.rs`) respects field types (`string`, `int`, `float`, `boolean`, `datetime`, `uuid`, `json`, `enum(...)`), nullability, uniqueness, and `range`/`length`/`pattern` constraints, and populates foreign keys from already-generated parent rows in dependency order.
- **Quality scoring** (`WorldGenerator.evaluate` / `GeneratedWorld.fidelity_score` / `.quality_report`) counts actual nullability, uniqueness, and constraint violations in the generated data and derives a fidelity score from them — it is not a hardcoded `1.0`.
- **Export** (`to_pandas`, `to_parquet`, `to_json`) operates on the real generated rows.
- **`DataQualityAnalyzer`** (`pysynthdata._core.DataQualityAnalyzer`) computes real missing/duplicate/outlier/temporal counts over row data you pass it. (`inconsistent_records` is always `0` — detecting logical inconsistency between semantically-related fields needs domain knowledge this generic analyzer doesn't have, so it's left unimplemented rather than faked.)
- **`DataGovernanceManager`** stores and returns the policies you give it; it makes no legal or compliance claims.
- **`PrivacyBudget` / `WorldGenerator.generate_private()`** (`src/privacy.rs`) — a real epsilon-delta budget (sequential composition: `spend()` errors rather than silently overspending) and a genuine Laplace-mechanism noise injector for every Int/Float field, calibrated from each field's declared range constraint (or observed value spread, if it has none) divided by the epsilon you spend. Noised values are clamped back to the field's declared range (a public, schema-known bound — safe post-processing under differential privacy, not an extra budget cost).
- **`WorldGenerator.generate_streaming()` / `.to_parquet_streaming()`** (`src/generator.rs`'s `generate_streaming`) — chunked generation with peak memory bounded by `chunk_size`, not `num_records`, for entities that aren't a foreign-key source for another entity (referenced "parent" entities are still fully materialized, which relational FK sampling genuinely requires). `to_parquet_streaming()` writes each chunk straight to a `pyarrow.parquet.ParquetWriter` row group with no full-entity intermediate structure.

## What's intentionally not here

An earlier version of this package shipped `GDPRCompliance`, `HIPAACompliance`, and `SOC2Compliance` classes whose methods (`check_consent`, `encrypt_phi`, `verify_access_controls`, ...) always returned success regardless of input — a compliance API that always says "compliant" is worse than no API, so it was deleted rather than kept as decoration. If you need actual GDPR/HIPAA/SOC2 compliance tooling, this package does not provide it. (`PrivacyBudget`/`generate_private()` above give you real differential-privacy noise injection, which is a narrower, different thing from certifying compliance with any specific regulation — `check_privacy_compliance()`'s MCP tool, which asks a broader "is this dataset GDPR/CCPA/HIPAA/PCI-DSS compliant" question, still honestly returns `not_implemented`.)

`pysynthdata/_mcp_tools.py` defines MCP-style tool schemas and async handlers following the same honesty rule: `generate_synthetic_dataset`, `estimate_data_quality`, and `export_synthetic_data` call the real generation engine above and return computed numbers. Tools that would require domain logic this codebase doesn't implement (PII detection, k-anonymity, fairness/bias auditing, ML-utility evaluation, cross-dataset distribution tests) return `{"status": "not_implemented"}` with a reason, instead of a plausible-looking fake number. **This file is not currently wired to a running server** — there is no MCP stdio/HTTP server entry point in this repo that loads it, no test imports it, and it ships in neither the PyPI wheel nor the sdist (`python-source = "python"` in `pyproject.toml` packages only `python/pysynthdata/`, not the top-level `pysynthdata/` directory). Treat it as unused reference code, not a working integration, until something actually starts a server from it.

## vs SDV

SDV (Synthetic Data Vault) is the leading OSS synthetic-tabular-data
library, and the closest comparison — but it solves a different problem
than PySynthData does. Tested both against the same real seed dataset
(the [Palmer Penguins](https://github.com/allisonhorst/palmerpenguins)
dataset, 333 complete real rows of actual biological measurements, dropped
rows with real missing values), generating 333 synthetic rows from each.

**The real architectural difference:** SDV's `GaussianCopulaSynthesizer`
*fits a real statistical model to your actual data* (real marginal
distributions, real cross-column correlations) and samples from it.
PySynthData has no fit step at all — you hand-declare a schema (types,
ranges, enum values, foreign keys) and it generates rows satisfying those
constraints, with no attempt to match a specific real dataset's
distribution shape. That difference shows directly in the numbers:

| | SDV (fit + sample) | PySynthData (schema-only) |
|---|---|---|
| Generation time | 0.10s (0.08s fit + 0.02s sample) | 0.000s (no fit step) |
| KS statistic vs real, numeric cols (lower = closer), avg of 5 | 0.091 | 0.149 |
| Total variation distance vs real, categorical cols, avg of 3 | 0.046 | 0.118 |
| real `corr(bill_length_mm, body_mass_g)` = 0.589 | 0.601 (preserved) | -0.045 (not preserved — each field is generated independently) |

SDV wins decisively on statistical fidelity to a specific real dataset,
including real cross-column correlation, because that's literally what it
was built to do. PySynthData is faster (no model to fit) and gives you
things SDV's single-table synthesizers don't: real multi-entity foreign-key
generation across entities in one call, a real epsilon-delta differential-
privacy budget (`generate_private()`), and chunked/streaming generation
bounded by `chunk_size` for datasets too large to hold in memory. If your
goal is "make data that statistically resembles this real dataset," use
SDV. If your goal is "make realistic-shaped relational data satisfying
constraints I declare, with real referential integrity and an optional
privacy budget," that's what PySynthData is actually for.

**Real bug found and fixed while running this benchmark:** every declared
`enum(...)` value was silently lowercased (`"Adelie"` → `"adelie"`) in
*both* of this codebase's two independent field-type parsers
(`src/parser.rs`'s YAML loader and `src/lib.rs`'s `Schema.add_field` Python
binding — the one every documented example in this README actually uses).
Root cause: both matched the `enum(` keyword against a lowercased copy of
the type string, then extracted the enum *values* from that same
lowercased copy instead of the original. This wasn't just a cosmetic
casing issue — it collapsed the categorical fidelity comparison above to
its worst possible value (total variation distance of 1.0, meaning zero
category overlap with real data) until fixed, because the generated
`"adelie"`/`"gentoo"`/`"chinstrap"` values shared no labels at all with the
real `"Adelie"`/`"Gentoo"`/`"Chinstrap"` categories they were supposed to
approximate. Fixed in both parsers (kept case-insensitive matching on the
`enum(` keyword itself, e.g. `ENUM(...)` still works); regression tests
added at both the Rust (`src/parser.rs`) and Python
(`python/tests/test_enum_casing.py`) level. The categorical TVD numbers in
the table above are post-fix; pre-fix they were 1.0/1.0 for the two
enum-typed columns.

## Roadmap

Implemented behind the Rust `pysynthdata` crate but not yet wired to the Python API: robotics fleet simulation (`robotics.rs`, `ros2_bridge.rs`), behavioral state machines and scenario branching (`behaviors.rs`), a domain knowledge base for schema inference (`research.rs`), drift/anomaly monitoring (`monitoring.rs`), "real world mess" / unconventional-data injectors (`real_world_mess.rs`, `unconventional_data.rs`), and multi-modal (vision/audio/sensor/text/temporal) data augmentation (`multimodal_augmentation.rs`). These have Rust-level test coverage but no Python bindings yet; binding them is future work, not a promised feature of the current release.

Also not yet real:
- Iceberg/Delta Lake output targets. `to_parquet_streaming()` (above) fixes the underlying memory-footprint problem for large batch generation, but only writes Parquet — no table-format writer for Iceberg or Delta Lake exists.
- Advanced composition for `PrivacyBudget` (only basic/sequential composition is implemented — real but conservative; a tighter accountant, e.g. Rényi DP, would allow more queries for the same total epsilon).

## Known issues

- `multimodal_augmentation.rs` (vision, audio, sensor, and temporal augmentors) was merged with Rust-level coverage but, like the other modules above, has no Python bindings yet — it is not usable from `import pysynthdata`.
- No open GitHub issues at the time of this writing.

## Development

```bash
# Build the Rust extension into your active virtualenv
pip install maturin
maturin develop --release

# Rust checks
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check

# Python checks
pip install -e ".[dev]"
ruff check python/ pysynthdata/
pytest
```

Most automated coverage is still the Rust `cargo test` suite above (CI runs it on every push). `python/tests/` now also has a real pytest suite covering `PrivacyBudget`/`generate_private()` and `generate_streaming()`/`to_parquet_streaming()` — the Python-layer wrapper logic (PyO3 boundary, threading/queue plumbing, pyarrow integration) those Rust unit tests can't reach.

## License

This project is licensed under the [Apache License 2.0](LICENSE).
