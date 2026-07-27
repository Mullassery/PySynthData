"""High-level Python API for World Compiler."""

from typing import Optional, Dict, Any
import json
from pathlib import Path
import pandas as pd
import pyarrow.parquet as pq

class Schema:
    """Represents a world schema (entities, relationships, constraints)."""

    def __init__(self, yaml_path: Optional[str] = None):
        """Initialize schema from YAML file or empty."""
        self.entities = {}
        self.relationships = []
        self.constraints = []

        if yaml_path:
            self.load_from_yaml(yaml_path)

    def add_entity(self, name: str, fields: Optional[Dict[str, str]] = None) -> None:
        """Add entity to schema."""
        self.entities[name] = fields or {}

    def add_relationship(self, from_entity: str, to_entity: str, cardinality: str) -> None:
        """Add relationship between entities."""
        self.relationships.append({
            "from": from_entity,
            "to": to_entity,
            "cardinality": cardinality,
        })

    def load_from_yaml(self, path: str) -> None:
        """Load schema from YAML file."""
        import yaml

        with open(path) as f:
            data = yaml.safe_load(f)

        if "entities" in data:
            self.entities = data["entities"]
        if "relationships" in data:
            self.relationships = data["relationships"]
        if "constraints" in data:
            self.constraints = data["constraints"]

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary."""
        return {
            "entities": self.entities,
            "relationships": self.relationships,
            "constraints": self.constraints,
        }


class WorldGenerator:
    """Generate synthetic worlds from a schema."""

    def __init__(self, schema: Schema):
        """Initialize generator with schema."""
        self.schema = schema
        self._records = {}

    def generate(self, num_records: int = 1000, seed: int = 42) -> "GeneratedWorld":
        """Generate synthetic world."""
        return GeneratedWorld(schema=self.schema, num_records=num_records, seed=seed)

    @classmethod
    def from_yaml(cls, yaml_path: str) -> "WorldGenerator":
        """Create generator from YAML schema."""
        schema = Schema(yaml_path)
        return cls(schema)


class GeneratedWorld:
    """Represents a generated synthetic world."""

    def __init__(self, schema: Schema, num_records: int = 1000, seed: int = 42):
        """Initialize generated world."""
        self.schema = schema
        self.num_records = num_records
        self.seed = seed
        self.data = {}

    def to_pandas(self, entity: str) -> pd.DataFrame:
        """Convert entity data to pandas DataFrame."""
        if entity not in self.data:
            raise ValueError(f"Entity {entity} not found in world")
        return pd.DataFrame(self.data[entity])

    def to_parquet(self, path: str, entity: Optional[str] = None) -> None:
        """Export world or entity to Parquet."""
        if entity:
            df = self.to_pandas(entity)
            df.to_parquet(f"{path}/{entity}.parquet")
        else:
            for entity_name, records in self.data.items():
                df = pd.DataFrame(records)
                df.to_parquet(f"{path}/{entity_name}.parquet")

    def to_json(self, path: str) -> None:
        """Export world to JSON."""
        with open(path, "w") as f:
            json.dump(self.data, f, default=str, indent=2)

    @property
    def fidelity_score(self) -> float:
        """Get world fidelity score (0-1)."""
        return 1.0  # Stub

    @property
    def constraint_violations(self) -> int:
        """Get number of constraint violations."""
        return 0  # Stub


def load_schema(path: str) -> Schema:
    """Load schema from file (auto-detect format)."""
    p = Path(path)
    if p.suffix == ".yaml" or p.suffix == ".yml":
        return load_schema_yaml(path)
    elif p.suffix == ".json":
        return load_schema_json(path)
    else:
        raise ValueError(f"Unsupported schema format: {p.suffix}")


def load_schema_yaml(path: str) -> Schema:
    """Load schema from YAML file."""
    return Schema(path)


def load_schema_json(path: str) -> Schema:
    """Load schema from JSON file."""
    schema = Schema()
    with open(path) as f:
        data = json.load(f)
    schema.entities = data.get("entities", {})
    schema.relationships = data.get("relationships", [])
    schema.constraints = data.get("constraints", [])
    return schema
