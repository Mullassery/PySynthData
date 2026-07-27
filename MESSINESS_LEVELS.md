# PySynthData: Messiness Levels

## 5 Levels of Data Chaos

### Level 1: Slightly Messy (2-5% affected records)
**Target:** Startup MVP, internal testing  
**Patterns Enabled:**
- 1-2% missing values (NULL chaos)
- 0.5% duplicates
- 1% fat-finger errors
- Realistic but mostly clean

**Config:**
```python
config = MessinessLevel.SLIGHTLY_MESSY
gen = RealWorldMessGenerator(seed=42)
gen.apply_level(data, config)
```

### Level 2: Moderately Messy (5-15% affected records)
**Target:** Early-stage production, legacy system integration  
**Patterns Enabled:**
- All Level 1 patterns
- 5% null chaos (multiple representations)
- 2% field swaps
- 3% unit confusion
- Schema version mixing starts

**Use Cases:**
- Your first migration from old system
- Early prod with some legacy cruft
- Normal production baseline

### Level 3: Very Messy (15-40% affected records)
**Target:** Production reality, real data migration  
**Patterns Enabled:**
- All Level 1-2 patterns
- 10% decimal separator chaos
- 5% timezone confusion
- Partial batch incompleteness
- 3% cascading errors
- Encoding mismatches

**Use Cases:**
- Real production data
- Multi-system integration
- After bad deploy (before fix)
- Sensor drift over weeks

### Level 4: Extremely Messy (40-70% affected records)
**Target:** Post-incident forensics, disaster recovery testing  
**Patterns Enabled:**
- All Level 1-3 patterns
- 20% encoding chaos
- 10% Y2K-style bugs
- Broken referential integrity
- Cache stale writes
- SQL update typos (WHERE 1=1 scenarios)
- Heavy cascading errors

**Use Cases:**
- Your system after a bad migration
- Post-incident data cleanup
- Stress testing your pipeline
- Testing disaster recovery

### Level 5: Nightmare Mode (70%+ affected records)
**Target:** Breaking point testing, resilience benchmarking  
**Patterns Enabled:**
- ALL chaos patterns simultaneously
- 30%+ null chaos
- 20% encoding failures
- 15% cascading errors
- 10% field swaps
- Everything broken at once

**Use Cases:**
- Pipeline stress testing
- "Will our system survive THIS?"
- Resilience benchmarking
- Breaking point identification

---

## Quick Selection Guide

**"My data looks clean"** → Level 1  
**"We just integrated legacy systems"** → Level 2  
**"This is real production data"** → Level 3  
**"We had a bad deploy/migration"** → Level 4  
**"I want to break my pipeline"** → Level 5  

---

## Example: Testing Data Pipeline

```python
from pysynthdata import MessinessLevel, RealWorldMessGenerator

def test_pipeline_resilience():
    """Test pipeline at different chaos levels"""
    
    for level in [MessinessLevel.SLIGHTLY_MESSY, 
                   MessinessLevel.MODERATELY_MESSY,
                   MessinessLevel.VERY_MESSY,
                   MessinessLevel.EXTREMELY_MESSY,
                   MessinessLevel.NIGHTMARE_MODE]:
        
        data = generate_base_data(100_000)
        gen = RealWorldMessGenerator(seed=42)
        gen.apply_level(data, level)
        
        # Run pipeline
        try:
            result = pipeline.process(data)
            accuracy = calculate_accuracy(result)
            print(f"{level}: {accuracy}% accuracy")
        except Exception as e:
            print(f"{level}: FAILED - {e}")

test_pipeline_resilience()
```

**Expected Output:**
```
Level 1 (Slightly Messy): 99.2% accuracy
Level 2 (Moderately Messy): 97.5% accuracy
Level 3 (Very Messy): 92.3% accuracy
Level 4 (Extremely Messy): FAILED - TypeError in type conversion
Level 5 (Nightmare Mode): FAILED - encoding error in third batch
```

**Interpretation:**
- Your pipeline handles Level 1-3 → you're ready for production
- Fails at Level 4 → you need better error handling
- Fails at Level 5 → you found your breaking point

---

## Messiness Levels: Detailed Pattern Breakdown

| Pattern | L1 | L2 | L3 | L4 | L5 |
|---------|----|----|----|----|-----|
| NULL chaos | 1% | 5% | 10% | 20% | 30% |
| Duplicates | 0.5% | 2% | 5% | 10% | 15% |
| Fat-finger errors | 1% | 2% | 3% | 5% | 10% |
| Field swaps | 0% | 1% | 2% | 5% | 10% |
| Unit confusion | 0% | 3% | 5% | 8% | 15% |
| Encoding mismatches | 0% | 0% | 2% | 10% | 25% |
| Decimal chaos | 0% | 0% | 10% | 15% | 20% |
| Timezone confusion | 0% | 0% | 5% | 10% | 20% |
| Y2K bugs | 0% | 0% | 0% | 10% | 15% |
| Partial batches | 0% | 2% | 5% | 15% | 25% |
| Schema mixing | 0% | 5% | 10% | 20% | 30% |
| Cascading errors | 0% | 0% | 3% | 10% | 20% |
| Orphaned FKs | 0% | 0% | 2% | 5% | 15% |
| SQL typos | 0% | 0% | 0% | 5% | 15% |

---

## Use in Your CI/CD

```yaml
# .github/workflows/pipeline_resilience.yml
name: Data Pipeline Resilience Test

on: [push]

jobs:
  test-resilience:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        messiness: [SLIGHTLY_MESSY, MODERATELY_MESSY, VERY_MESSY]
    steps:
      - uses: actions/checkout@v2
      - name: Test at messiness level ${{ matrix.messiness }}
        run: |
          python test_pipeline.py --messiness ${{ matrix.messiness }}
```

---

## Production Readiness Criteria

✅ **Level 1-2 Pass** → Pre-production ready  
✅ **Level 1-3 Pass** → Production ready  
⚠️ **Level 1-3 Pass, Level 4 Fail** → Production ready with caveats (need monitoring)  
❌ **Level 3+ Fail** → NOT production ready  

Your pipeline's messiness tolerance is your confidence level.

