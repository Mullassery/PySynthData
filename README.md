# PySynthData

**Generate synthetic relational datasets from a schema.**

Define entities, fields, foreign-key relationships, and constraints; get back real, typed rows — with foreign keys that actually point at rows that exist, constraints that are actually enforced, and a quality report that actually measures violations instead of returning a hardcoded number. Row generation and quality scoring happen in a compiled Rust core; the Python layer is a thin wrapper around it.

[![Python 3.10+](https://img.shields.io/badge/Python-3.10%2B-blue)](https://www.python.org)
[![License: Proprietary](https://img.shields.io/badge/License-Proprietary-blue.svg)](./LICENSE)

---

## Status

Early (`0.3.0`). The core pipeline — schema definition, row generation, foreign keys, constraints, quality scoring, and pandas/Parquet/JSON export — is implemented and tested end to end. Several bundled Rust modules (robotics fleet simulation, ROS2 bridge, behavioral state machines, domain research knowledge base, monitoring/drift detection, "real world mess" injectors) exist in the codebase but are **not yet exposed through the Python API** — see [Roadmap](#roadmap).

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

## What's real here

- **Row generation** (`src/generator.rs`) respects field types (`string`, `int`, `float`, `boolean`, `datetime`, `uuid`, `json`, `enum(...)`), nullability, uniqueness, and `range`/`length`/`pattern` constraints, and populates foreign keys from already-generated parent rows in dependency order.
- **Quality scoring** (`WorldGenerator.evaluate` / `GeneratedWorld.fidelity_score` / `.quality_report`) counts actual nullability, uniqueness, and constraint violations in the generated data and derives a fidelity score from them — it is not a hardcoded `1.0`.
- **Export** (`to_pandas`, `to_parquet`, `to_json`) operates on the real generated rows.
- **`DataQualityAnalyzer`** (`pysynthdata._core.DataQualityAnalyzer`) computes real missing/duplicate/outlier/temporal counts over row data you pass it. (`inconsistent_records` is always `0` — detecting logical inconsistency between semantically-related fields needs domain knowledge this generic analyzer doesn't have, so it's left unimplemented rather than faked.)
- **`DataGovernanceManager`** stores and returns the policies you give it; it makes no legal or compliance claims.

## What's intentionally not here

An earlier version of this package shipped `GDPRCompliance`, `HIPAACompliance`, and `SOC2Compliance` classes whose methods (`check_consent`, `encrypt_phi`, `verify_access_controls`, ...) always returned success regardless of input — a compliance API that always says "compliant" is worse than no API, so it was deleted rather than kept as decoration. If you need actual GDPR/HIPAA/SOC2 compliance tooling, this package does not provide it.

The MCP tool handlers in `pysynthdata/_mcp_tools.py` follow the same rule: `generate_synthetic_dataset`, `estimate_data_quality`, and `export_synthetic_data` are backed by the real generation engine above. Tools that would require domain logic this codebase doesn't implement (PII detection, k-anonymity, differential privacy, fairness/bias auditing, ML-utility evaluation, cross-dataset distribution tests) return `{"status": "not_implemented"}` with a reason, instead of a plausible-looking fake number.

## Roadmap

Implemented behind the Rust `pysynthdata` crate but not yet wired to the Python API: robotics fleet simulation (`robotics.rs`, `ros2_bridge.rs`), behavioral state machines and scenario branching (`behaviors.rs`), a domain knowledge base for schema inference (`research.rs`), drift/anomaly monitoring (`monitoring.rs`), and "real world mess" / unconventional-data injectors (`real_world_mess.rs`, `unconventional_data.rs`). These have Rust-level test coverage but no Python bindings yet; binding them is future work, not a promised feature of the current release.

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
pytest tests_python/ -v
ruff check python/ pysynthdata/
```

## License

Proprietary — free to use with explicit attribution. See [LICENSE](./LICENSE).
