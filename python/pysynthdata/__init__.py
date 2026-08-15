"""PySynthData: Generate real synthetic data from a schema.

`Schema` and `WorldGenerator` are thin Python wrappers around the compiled
Rust `_core` extension, which does the actual row generation and quality
scoring.
"""

__version__ = "0.3.0"

from pysynthdata import _core
from pysynthdata.api import (
    WorldGenerator,
    Schema,
    GeneratedWorld,
    load_schema,
    load_schema_yaml,
    load_schema_json,
)

__all__ = [
    "WorldGenerator",
    "Schema",
    "GeneratedWorld",
    "load_schema",
    "load_schema_yaml",
    "load_schema_json",
    "_core",
]
