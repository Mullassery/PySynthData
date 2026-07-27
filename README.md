# PySynthData: Synthetic World Generation Platform

Generate complete synthetic worlds from schemas, data, or natural language. Production-ready with compliance, governance, and enterprise features.

**Status**: ✅ Production Ready (v0.2.0) | **LOC**: 4,900+ | **Modules**: 16 | **Python**: 3.10+

## What It Does

**Generate realistic synthetic worlds** for:
- 🎯 AI training data (GDPR/HIPAA compliant)
- 🤖 Robotics fleet simulation (100+ robots, ROS2-native)
- 🧪 Data pipeline stress testing (5 calibrated chaos levels)
- 📊 Enterprise digital twins
- 🔬 Reproducible research datasets

### Core Features

✅ **Schema → Synthetic Data**: YAML/JSON schemas generate realistic records with referential integrity  
✅ **Temporal Behaviors**: Entity state machines, event sequences, edge case synthesis  
✅ **Robotics Ready**: Fleet coordination, collision detection, sensor simulation, ROS2 integration  
✅ **Domain Intelligence**: 5 pre-configured domains + custom domain support  
✅ **Production Chaos**: 35+ realistic data quality patterns (missing values, encoding errors, cascading failures)  
✅ **Enterprise Features**: Compliance frameworks (GDPR/HIPAA/SOC2), audit logging, lineage tracking, regulatory reporting  
✅ **Quality Monitoring**: Drift detection, anomaly detection, performance tracking  
✅ **Cost Transparent**: Automatic cost estimation and performance profiling  

---

## Quick Start

### Install

```bash
pip install pysynthdata
```

### Generate Synthetic Banking World

```python
from pysynthdata import WorldGenerator, DomainKnowledgeBase

# Use pre-configured Banking domain
kb = DomainKnowledgeBase()
schema = kb.get_domain("Banking").schema

# Generate 1M synthetic customers, accounts, transactions
gen = WorldGenerator(schema)
world = gen.generate(num_records=1_000_000, seed=42)

# Export to Parquet
world.to_parquet("synthetic_banking/")
```

### Add Realistic Messiness

```python
from pysynthdata import RealWorldMessGenerator, MessinessLevel

# Generate data that looks like production
data = generate_base_data()
gen = RealWorldMessGenerator(seed=42)
gen.apply_level(data, MessinessLevel.LEVEL_3)  # 15-40% affected

# Test if your pipeline survives
try:
    result = pipeline.process(data)
    print("✅ Pipeline handles Level 3 chaos")
except Exception as e:
    print(f"❌ Pipeline fails: {e}")
```

### Monitor Quality & Drift

```python
from pysynthdata import DriftDetector, AnomalyDetector, MonitoringConfig

# Enable comprehensive monitoring
config = MonitoringConfig(
    track_schema_drift=True,
    track_data_drift=True,
    track_constraint_violations=True,
    track_edge_cases=True
)

# Generate compliance report
gdpr_report = world.get_compliance_report(ComplianceFramework.GDPR)
print(f"GDPR Status: {gdpr_report.compliance_status}")
```

### Simulate Robot Fleet

```python
from pysynthdata import FleetSimulation, Environment, Robot

env = Environment(name="Warehouse", type="warehouse", width=100, height=100)
sim = FleetSimulation(env)

# Add 100 robots
for i in range(100):
    robot = Robot(id=f"robot_{i}", type="mobile_base", battery=100)
    sim.add_robot(robot)

# Simulate operations and generate ROS2 sensor streams
simulation = sim.run_simulation(duration_hours=1_000_000)
```

---

## 5 Calibrated Messiness Levels

Test your pipeline at increasing chaos levels:

- **Level 1**: Slightly Messy (2-5% affected) — MVP testing
- **Level 2**: Moderately Messy (5-15% affected) — Early production
- **Level 3**: Very Messy (15-40% affected) — Real production data
- **Level 4**: Extremely Messy (40-70% affected) — Post-incident recovery
- **Level 5**: Nightmare Mode (70%+ affected) — Breaking point testing

```python
for level in [MessinessLevel.LEVEL_1, MessinessLevel.LEVEL_3, MessinessLevel.LEVEL_5]:
    data = generate_synthetic_world()
    gen.apply_level(data, level)
    
    try:
        result = pipeline.process(data)
        print(f"{level}: PASS")
    except Exception as e:
        print(f"{level}: FAIL - {e}")
```

---

## Enterprise Features

### Compliance Ready
- ✅ **GDPR**: Auto-anonymization, consent tracking, audit trail
- ✅ **HIPAA**: PHI encryption, access controls, breach notification
- ✅ **SOC2**: Change management, monitoring, incident response
- ✅ **Custom**: Extend to any compliance framework

### Data Governance
- Lineage tracking (source → transformations → audit)
- Policy enforcement (access control, retention)
- Reproducibility guarantees (fixed seed = exact regeneration)
- Cost estimation (per operation, per record)

### Production Data Patterns (35+)
- Legacy system cruft (mixed ID formats, NULL chaos)
- Human errors (typos, field swaps, copy-paste truncation)
- System failures (encoding mismatches, timeouts, cascading errors)
- Temporal chaos (timezone confusion, Y2K bugs, daylight savings)
- Migration artifacts (partial loads, schema mixing, broken FKs)

---

## Documentation

- **[Quick Start](QUICK_START.md)** — 5-minute examples
- **[Messiness Levels](MESSINESS_LEVELS.md)** — Calibrated chaos for testing
- **[Enterprise Features](ENTERPRISE_FEATURES.md)** — Compliance, governance, audit
- **[Phase 2-4 Guide](PHASE_2_3_4_GUIDE.md)** — Architecture deep-dive
- **[Complete Platform](COMPLETE_PLATFORM.md)** — Full feature inventory

---

## Building & Development

```bash
# Build Rust core
cargo build --release

# Build Python wheels
pip install maturin
maturin develop              # Development install
maturin build --release      # Production wheel

# Run tests
cargo test
pytest tests/
```

## Supported Domains

| Domain | Entities | Relationships | Patterns | Status |
|--------|----------|---------------|----------|--------|
| **Banking** | Customer, Account, Transaction, Merchant | 4 | Fraud, churn, rapid activity | ✅ |
| **Insurance** | Policy, Claim, Underwriter | 3 | Claims clustering, catastrophe | ✅ |
| **Healthcare** | Patient, Doctor, Treatment | 3 | Rare diseases, complications | ✅ |
| **Manufacturing** | Equipment, Process, Quality | 3 | Failures, maintenance cycles | ✅ |
| **Robotics** | Robot, Task, Sensor | 3 | Localization failure, battery | ✅ |

---

## Performance

| Operation | Throughput | Latency |
|-----------|-----------|---------|
| Generate 1M records | 200K records/sec | <5 min |
| Simulate 1K entity timelines | 1M events/sec | <2 min |
| Fleet simulation (100 robots) | Real-time | <100ms per frame |
| Schema inference from NL | — | <1 sec |
| Anomaly detection | — | <100ms per batch |

---

## License

Apache 2.0 - See [LICENSE](LICENSE) for details.

## Contributing

PRs welcome for:
- New domains & patterns
- Performance optimizations
- Additional compliance frameworks
- Documentation & examples
- Bug reports & feature requests

## Citation

If you use PySynthData in research, please cite:

```bibtex
@software{pysynthdata2026,
  author = {Mullassery, Georgi},
  title = {PySynthData: Synthetic World Generation Platform},
  year = {2026},
  url = {https://github.com/Mullassery/PySynthData}
}
```

---

**Questions?** Open an issue on [GitHub](https://github.com/Mullassery/PySynthData) or email mullassery@gmail.com
