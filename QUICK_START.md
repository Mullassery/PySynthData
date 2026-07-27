# PySynthData: Quick Start

## Installation

```bash
pip install pysynthdata
```

## 5-Minute Example

### 1. Generate Banking Schema

```python
from pysynthdata import WorldGenerator, Schema, DomainKnowledgeBase

# Use pre-configured Banking domain
kb = DomainKnowledgeBase()
banking = kb.get_domain("Banking")

# Create schema
schema = Schema()
schema.add_entity("Customer", {"id": "uuid", "name": "string", "status": "active|suspended|closed"})
schema.add_entity("Account", {"id": "uuid", "customer_id": "uuid", "balance": "float"})
schema.add_relationship("Customer", "Account", "1:n")
```

### 2. Generate Synthetic World

```python
gen = WorldGenerator(schema)
world = gen.generate(num_records=10_000, seed=42)

# Export to Parquet
world.to_parquet("synthetic_data/")
```

### 3. Add Realistic Messiness

```python
from pysynthdata import DataQualityDegradation, DataQualityConfig

# Configure realistic data quality issues
quality_config = DataQualityConfig(
    missing_value_rate=0.02,      # 2% missing values
    duplicate_rate=0.005,          # 0.5% duplicates
    outlier_rate=0.03,             # 3% outliers
    typo_rate=0.01,                # 1% typos
    inconsistency_rate=0.02,       # 2% logical inconsistencies
    temporal_inconsistency_rate=0.01  # 1% date/time issues
)

# Apply degradation
degrader = DataQualityDegradation(seed=42)
degrader.apply_quality_degradation(data, quality_config)
```

### 4. Monitor Quality

```python
from pysynthdata import DriftDetector, AnomalyDetector, PerformanceMonitor, MonitoringConfig

# Enable monitoring
monitoring = MonitoringConfig(
    track_schema_drift=True,
    track_data_drift=True,
    track_constraint_violations=True,
    track_edge_cases=True,
    track_performance_metrics=True,
)

# Detect anomalies
detector = AnomalyDetector()
detector.detect_minor_anomaly("cust_1", "balance", "100.00", "100.00000001", 0.8, 1000)
detector.detect_format_anomaly("cust_2", "email", "user@example.com", "invalid_email", 1000)

# Get summary
summary = detector.get_anomaly_summary()
print(f"Total anomalies: {summary.total_anomalies}")
print(f"Critical: {summary.critical_count}, Warning: {summary.warning_count}, Info: {summary.info_count}")
```

### 5. Robotics Simulation (Bonus)

```python
from pysynthdata import FleetSimulation, Environment, Robot, FleetCoordinator, AllocationStrategy

# Create warehouse
env = Environment(
    name="Warehouse A",
    env_type="warehouse",
    width=100.0, height=100.0,
)

# Initialize fleet
sim = FleetSimulation(env)
for i in range(50):
    robot = Robot(
        id=f"robot_{i}",
        robot_type="mobile_base",
        base_x=i*2, base_y=0,
        battery=100
    )
    sim.add_robot(robot)

# Coordinate tasks
coordinator = FleetCoordinator("greedy")
allocation = coordinator.allocate_tasks(sim, tasks)
```

## Common Patterns

### Banking Simulation
```python
kb = DomainKnowledgeBase()
banking = kb.get_domain("Banking")
# ... generate customer, account, transaction data with realistic fraud patterns
```

### Insurance Claims
```python
insurance = kb.get_domain("Insurance")
# ... generate policies, claims, underwriting workflows
```

### Robotics Fleet
```python
robotics = kb.get_domain("Robotics")
# ... generate robot fleets, task allocation, sensor streams
```

### Healthcare Patient Data
```python
healthcare = kb.get_domain("Healthcare")
# ... generate patient records, treatments, outcomes (HIPAA-safe)
```

### Manufacturing Operations
```python
manufacturing = kb.get_domain("Manufacturing")
# ... generate equipment failures, maintenance schedules, downtime
```

## API Reference

### Core Classes

- `WorldGenerator` — Generate synthetic worlds from schemas
- `Schema` — Domain model definitions
- `DomainKnowledgeBase` — Pre-loaded domain research
- `BehaviorSimulator` — Entity lifecycle simulation
- `FleetSimulation` — Robotics fleet management
- `ROS2SimulatorBridge` — ROS2 topic publishing
- `DriftDetector` — Schema/data drift monitoring
- `AnomalyDetector` — Anomaly detection (minor/major/format)
- `PerformanceMonitor` — Latency & throughput tracking
- `DataQualityDegradation` — Realistic messiness injection

### Data Quality Features

- Missing values (clustered)
- Duplicates
- Outliers (extreme, impossible, sparse, clustered)
- Typos
- Cross-field inconsistencies
- Temporal inconsistencies

### Monitoring Capabilities

- Schema drift detection
- Data drift detection
- Constraint violation tracking
- Edge case frequency monitoring
- Anomaly classification (5 types)
- Performance profiling
- Alert management

## Next Steps

1. Read `PHASE_2_3_4_GUIDE.md` for detailed architecture
2. Check `examples/banking_schema.yaml` for schema format
3. Explore `tests/` for more usage patterns
4. Deploy as managed service (Phase 5)

---

**Ready to generate synthetic worlds?**

