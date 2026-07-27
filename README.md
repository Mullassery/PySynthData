# PySynthData

Generate complete synthetic worlds from schemas, data, APIs, or natural language.

**Status**: Phase 1 Development (Schema → Data Generation)

## Vision

Move beyond synthetic data rows to complete, realistic operational universes. PySynthData autonomously constructs living environments with:

- **Entities**: Realistic objects with distributions and constraints
- **Relationships**: Preserved cardinalities and referential integrity
- **Behaviors**: State machines, temporal sequences, interactions
- **Edge Cases**: Rare events, anomalies, adversarial scenarios
- **Scenarios**: Branching timelines, what-if environments

## Quick Start

### Install

```bash
pip install pysynthdata
```

### Use

```python
from pysynthdata import WorldGenerator, Schema

# Create schema
schema = Schema()
schema.add_entity("Customer", {"id": "uuid", "name": "string", "email": "string"})
schema.add_entity("Account", {"id": "uuid", "customer_id": "uuid", "balance": "float"})
schema.add_relationship("Customer", "Account", "1:n")

# Generate world
gen = WorldGenerator(schema)
world = gen.generate(num_records=1_000_000)

# Export
world.to_parquet("synthetic_world/")
```

### From YAML

```python
gen = WorldGenerator.from_yaml("schema.yaml")
world = gen.generate(num_records=1_000_000, seed=42)
```

## Schema Definition

Define domains in YAML:

```yaml
entities:
  Customer:
    fields:
      id:
        type: uuid
        unique: true
      name:
        type: string
      email:
        type: string
        unique: true
      status:
        type: enum(active, suspended, closed)
    primary_key: id

  Account:
    fields:
      id:
        type: uuid
      customer_id:
        type: uuid
      account_type:
        type: enum(checking, savings, investment)
      balance:
        type: float
    primary_key: id

relationships:
  - from_entity: Customer
    to_entity: Account
    from_field: id
    to_field: customer_id
    cardinality: 1:n
```

## Roadmap

| Phase | Timeline | Focus |
|-------|----------|-------|
| **1** | Weeks 1-12 | Schema → data generation |
| **2** | Weeks 13-24 | Behaviors + temporal sequences |
| **3** | Weeks 25-36 | Robotics + ROS2 integration |
| **4** | Weeks 37-48 | LLM-powered research + NL input |
| **5** | Weeks 49-72 | Cloud platform + SaaS |

## Architecture

**Rust Backend** (performance, safety):
- Schema parsing (YAML/JSON)
- Distribution engine (realistic data synthesis)
- Constraint validation
- Parallel generation

**Python Wrapper** (usability):
- High-level API
- Data export (Parquet, JSON, Pandas)
- Schema utilities
- Examples and tutorials

## Building

```bash
# Build Rust
cargo build --release

# Build Python wheels
pip install maturin
maturin develop

# Run tests
cargo test
pytest tests/
```

## Contributing

Open source, Apache 2.0 license.

Early stage—PRs welcome for:
- Distribution engine improvements
- New constraint types
- Export formatters
- Documentation
- Examples

## License

Apache 2.0
