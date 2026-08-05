# PySynthData

**Generate realistic synthetic data at scale. Keep your real data private.**

Create privacy-preserving datasets for AI training, testing, and demos without exposing sensitive information. Use real patterns without real risk.

[![PyPI](https://img.shields.io/pypi/v/pysynthdata)](https://pypi.org/project/pysynthdata)
[![Python 3.10+](https://img.shields.io/badge/Python-3.10%2B-blue)](https://www.python.org)
[![License: Proprietary](https://img.shields.io/badge/License-Proprietary-blue.svg)](./LICENSE)

---

## 30-Second Start

```python
from pysynthdata import Synthesizer

# Learn patterns from real data
synth = Synthesizer.from_dataframe(real_df)

# Generate synthetic data (same patterns, no real data)
synthetic_df = synth.generate(rows=10000)

# Safe to share, train models, run demos
print(f"Generated {len(synthetic_df)} rows - no privacy concerns")
```

---

## Why PySynthData?

**The Problem:**
- Real datasets contain sensitive data (PII, financial, health)
- Can't share data for AI training or testing
- Demos expose live data
- Compliance requires data anonymization

**The Solution:**
- Generate synthetic data with same statistical properties
- No personally identifiable information
- Scale to any size needed
- Share freely for training, testing, demos

## Version History

### v2.0.0 (Current)
- ✅ MCP 2.0 Support
- ✅ Integrated with 17 other projects
- ✅ 207 unified MCP tools
- ✅ Intelligent orchestration
- ✅ Production-ready (wheels only)

## License

MIT

---

**MCP 2.0 Mega-Platform | v2.0.0 | Wheels-Only Distribution**
