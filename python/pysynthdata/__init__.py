"""PySynthData: Generate real synthetic data from a schema.

`Schema` and `WorldGenerator` are thin Python wrappers around the compiled
Rust `_core` extension, which does the actual row generation and quality
scoring.
"""

__version__ = "0.5.2"

from pysynthdata import _core
from pysynthdata._core import (
    AuditEvent,
    AuditEventType,
    DataGovernanceManager,
    DataGovernancePolicy,
    DataQualityAnalyzer,
    DataQualityMetrics,
    OutlierPattern,
)
from pysynthdata.api import (
    WorldGenerator,
    Schema,
    GeneratedWorld,
    PrivacyBudget,
    load_schema,
    load_schema_yaml,
    load_schema_json,
)

__all__ = [
    "AuditEvent",
    "AuditEventType",
    "DataGovernanceManager",
    "DataGovernancePolicy",
    "DataQualityAnalyzer",
    "DataQualityMetrics",
    "OutlierPattern",
    "WorldGenerator",
    "Schema",
    "GeneratedWorld",
    "PrivacyBudget",
    "load_schema",
    "load_schema_yaml",
    "load_schema_json",
    "_core",
]
