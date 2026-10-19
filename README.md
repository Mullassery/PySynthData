# PySynthData

A Rust-native library for generating synthetic relational datasets from a schema: real row generation with foreign keys, constraints, differential privacy support, and quality scoring.

## What it is

PySynthData generates realistic synthetic relational data from a declarative schema. It ensures foreign keys point to valid parents, enforces constraints, scores data quality, and exports to common formats. Core generation and quality scoring run in Rust with a Python API.

## Features / Highlights

- **Schema-driven generation**: Define entities, fields, relationships, and constraints (YAML or Python API).
- **Relational integrity**: Foreign keys are populated from existing parent rows in dependency order.
- **Constraint enforcement**: Supports nullability, uniqueness, ranges, lengths, patterns, and enums.
- **Differential privacy**: Laplace mechanism noise injection with a real ε-δ budget that prevents overspending.
- **Streaming generation**: Generate large datasets in chunks to bound peak memory.
- **Quality scoring**: Computes fidelity based on actual violations (not hardcoded).
- **Export formats**: Pandas DataFrames, Parquet, and JSON.
- **Schema inference**: Infer schemas from live Postgres, MySQL, or SQLite databases via SQLAlchemy.

## Who it's for / Use cases

- **Testing and demos**: Generate realistic relational fixtures without sharing production data.
- **Privacy-safe development**: Create synthetic derivatives of sensitive data with differential privacy.
- **Database prototyping**: Populate schemas with consistent relational data for development.
- **CI/testing**: Produce deterministic, reproducible test datasets from versioned schemas.

## What it is not

- **Not a full SDV replacement for all use cases**: Focused on relational schema fidelity, constraints, and privacy primitives; advanced ML-based synthesis is out of scope.
- **Not compliance certification**: Differential privacy is a technical primitive, not a guarantee of regulatory compliance.
- **Alpha maturity**: Some bundled components exist in Rust but are not exposed through the Python API (see roadmap).

## Project status

Version 0.5.0 (published to PyPI). Core pipeline (schema definition, row generation, FKs, constraints, quality scoring, exports, differential privacy, streaming, schema inference) is implemented and tested. See [ROADMAP_HONEST.md](ROADMAP_HONEST.md) for unexposed work and limitations.

## Installation

```bash
pip install pysynthdata
```

With database schema inference support:

```bash
pip install pysynthdata[db]
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
schema.add_relationship("customers", "orders", "id", "customer_id", "1:n")

generator = WorldGenerator(schema)
world = generator.generate(num_records=1000, seed=42)

df = world.to_pandas("customers")
world.to_parquet("out/")
world.to_json("out/world.json")

print(world.fidelity_score)
print(world.quality_report)
```

Generation is deterministic: same schema + seed produces identical results. Foreign keys always reference valid parent rows.

## Usage

### From YAML

```python
generator = WorldGenerator.from_yaml("examples/banking_schema.yaml")
world = generator.generate(num_records=5000, seed=7)
```

### Schema inference from database

```bash
pysynthdata infer-schema --db-url "postgresql+psycopg://user:pass@host/dbname" \
    --table customers --output customers_schema.yaml

pysynthdata infer-schema --db-url "mysql+pymysql://user:pass@host/dbname" \
    --all-tables --output full_schema.yaml
```

Drivers (psycopg/psycopg2, pymysql) must be installed separately. Output YAML is compatible with `Schema.from_yaml()`/`WorldGenerator.from_yaml()`.

### Differential privacy

```python
from pysynthdata import PrivacyBudget

budget = PrivacyBudget(epsilon=1.0, delta=1e-5)
private_world = generator.generate_private(num_records=1000, seed=42, budget=budget, epsilon=0.3)
print(private_world.privacy_report)
print(budget.remaining_epsilon)
```

### Streaming generation

```python
for entity_name, rows in generator.generate_streaming(num_records=5_000_000, seed=1, chunk_size=50_000):
    ...

generator.to_parquet_streaming(num_records=5_000_000, seed=1, out_dir="out/", chunk_size=50_000)
```

### Data quality analysis

```python
from pysynthdata import DataQualityAnalyzer

analyzer = DataQualityAnalyzer()
df = world.to_pandas("customers")
report = analyzer.analyze(df, entity_name="customers")
print(report)
```

## Known issues and deferred work

**Blocked / failed:**
- Linux/Windows wheels, expose Rust components, SDV fidelity/privacy benchmark.

**Rejected / deferred:**
- SDV/CTGAN parity.

Full shipped/blocked/rejected detail: [`QUEUE_STATUS.md`](QUEUE_STATUS.md).

## Documentation

- [Examples](examples/)
- [Roadmap & honest limitations](ROADMAP_HONEST.md)
- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)
- [Code of conduct](CODE_OF_CONDUCT.md)

## Related projects

Part of the Mullassery open-source family:

- [PyDependencyCheck](https://github.com/Mullassery/PyDependencyCheck) — dependency intelligence and supply-chain integrity
- [PyDBTGuard](https://github.com/Mullassery/PyDBTGuard) — dbt manifest analysis (risky tests, blast radius, cost)
- [PyAirflowTester](https://github.com/Mullassery/PyAirflowTester) — Airflow and dbt reliability platform
- [StatGuardian](https://github.com/Mullassery/StatGuardian) — declarative data-quality framework
- [PySynthData](https://github.com/Mullassery/PySynthData) — synthetic data generation for ML
- [PyReverseETL](https://github.com/Mullassery/PyReverseETL) — quality-validated reverse ETL
- [PyWeatherEnriched](https://github.com/Mullassery/PyWeatherEnriched) — weather-enriched feature engineering
- [Browse all projects](https://github.com/Mullassery?tab=repositories)

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for details.
