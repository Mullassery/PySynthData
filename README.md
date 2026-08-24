# PySynthData

**Generate synthetic relational datasets from a schema.**

Define entities, fields, foreign-key relationships, and constraints; get back real, typed rows — with foreign keys that actually point at rows that exist, constraints that are actually enforced, and a quality report that actually measures violations instead of returning a hardcoded number. Row generation and quality scoring happen in a compiled Rust core; the Python layer is a thin wrapper around it.

[![Python 3.10+](https://img.shields.io/badge/Python-3.10%2B-blue)](https://www.python.org)
[![License: Proprietary](https://img.shields.io/badge/License-Proprietary-blue.svg)](./LICENSE)

---

## Status

Early (`0.4.0`). The core pipeline — schema definition, row generation, foreign keys, constraints, quality scoring, pandas/Parquet/JSON export, differential-privacy noise injection, and chunked/streaming generation — is implemented and tested end to end. Several bundled Rust modules (robotics fleet simulation, ROS2 bridge, behavioral state machines, domain research knowledge base, monitoring/drift detection, "real world mess" injectors) exist in the codebase but are **not yet exposed through the Python API** — see [Roadmap](#roadmap).

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

The MCP tool handlers in `pysynthdata/_mcp_tools.py` follow the same rule: `generate_synthetic_dataset`, `estimate_data_quality`, and `export_synthetic_data` are backed by the real generation engine above. Tools that would require domain logic this codebase doesn't implement (PII detection, k-anonymity, fairness/bias auditing, ML-utility evaluation, cross-dataset distribution tests) return `{"status": "not_implemented"}` with a reason, instead of a plausible-looking fake number.

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

Proprietary — free to use with explicit attribution. See [LICENSE](./LICENSE).
