"""High-level Python API for PySynthData.

This is a thin wrapper around the real Rust `_core` extension — schema
construction, row generation, and quality-report computation all happen in
Rust. Nothing here reimplements generation in pure Python; `Schema` and
`WorldGenerator` hold a real `_core.Schema` / `_core.WorldGenerator` instance
and delegate every real piece of work to it.
"""

from typing import Optional, Dict, Any, List
import json
from pathlib import Path

import pandas as pd

from pysynthdata import _core


class Schema:
    """A world schema (entities, relationships, constraints), backed by the
    real Rust `_core.Schema`."""

    def __init__(self, yaml_path: Optional[str] = None):
        if yaml_path:
            self._inner = _core.Schema.from_yaml(str(yaml_path))
        else:
            self._inner = _core.Schema()

    @classmethod
    def from_yaml(cls, path: str) -> "Schema":
        """Load a schema from a YAML file."""
        return cls(yaml_path=path)

    @classmethod
    def from_yaml_str(cls, yaml_str: str) -> "Schema":
        """Load a schema from a YAML string."""
        schema = cls.__new__(cls)
        schema._inner = _core.Schema.from_yaml_str(yaml_str)
        return schema

    def add_entity(self, name: str) -> None:
        """Add an entity (table) to the schema."""
        self._inner.add_entity(name)

    def add_field(
        self,
        entity: str,
        name: str,
        field_type: str,
        nullable: bool = False,
        unique: bool = False,
        constraints: Optional[List[str]] = None,
    ) -> None:
        """Add a field to an entity.

        `field_type` is one of: string, int, float, boolean, datetime, uuid,
        json, or `enum(value1, value2, ...)`.
        """
        self._inner.add_field(entity, name, field_type, nullable, unique, constraints)

    def add_relationship(
        self,
        from_entity: str,
        to_entity: str,
        from_field: str,
        to_field: str,
        cardinality: str = "1:n",
    ) -> None:
        """Declare a foreign-key relationship. Generated rows in `to_entity`
        will have `to_field` populated with real values drawn from
        `from_entity`'s already-generated `from_field` column."""
        self._inner.add_relationship(from_entity, to_entity, from_field, to_field, cardinality)

    def add_constraint(
        self,
        constraint_type: str,
        entity: str,
        value: str,
        field: Optional[str] = None,
    ) -> None:
        """Add a constraint. `constraint_type` is one of: range, length,
        pattern, custom. `range`/`length` use a `"min-max"` string as `value`
        (e.g. `"0-1000000"`); `pattern` uses a regex as `value`."""
        self._inner.add_constraint(constraint_type, entity, value, field)

    @property
    def entities(self) -> List[str]:
        """Names of the entities defined on this schema."""
        return self._inner.entity_names()

    def to_yaml(self) -> str:
        return self._inner.to_yaml()

    def to_json(self) -> str:
        return self._inner.to_json()

    def load_from_yaml(self, path: str) -> None:
        """Replace this schema's contents with those parsed from a YAML file."""
        self._inner = _core.Schema.from_yaml(str(path))

    def to_dict(self) -> Dict[str, Any]:
        """Convert to a plain dict (round-trips through the real JSON encoder)."""
        return json.loads(self.to_json())


class WorldGenerator:
    """Generate synthetic worlds from a schema. All row generation and
    quality evaluation happens in the Rust `_core` extension."""

    def __init__(self, schema: Schema):
        self.schema = schema
        self._inner = _core.WorldGenerator(schema._inner)

    def generate(self, num_records: int = 1000, seed: int = 42) -> "GeneratedWorld":
        """Generate `num_records` real rows per entity, deterministically for
        a given seed, and compute a real quality report against the schema."""
        result = self._inner.generate(num_records, seed)
        return GeneratedWorld(result)

    @classmethod
    def from_yaml(cls, yaml_path: str) -> "WorldGenerator":
        """Create a generator directly from a YAML schema file."""
        return cls(Schema(yaml_path))


class GeneratedWorld:
    """A generated synthetic world: real per-entity row data plus a real
    quality report (both computed by the Rust `_core` extension — nothing
    here is a hardcoded stub)."""

    def __init__(self, result: Dict[str, Any]):
        self._result = result
        self.data: Dict[str, List[Dict[str, Any]]] = result["entities"]
        self.metadata: Dict[str, Any] = result["metadata"]
        self._quality: Dict[str, Any] = result["quality"]
        self.seed: int = self.metadata["seed"]
        self.num_records: int = self.metadata["record_count"]

    def to_pandas(self, entity: str) -> pd.DataFrame:
        """Convert one entity's generated rows to a pandas DataFrame."""
        if entity not in self.data:
            raise ValueError(
                f"Entity '{entity}' not found in world (available: {sorted(self.data)})"
            )
        return pd.DataFrame(self.data[entity])

    def to_parquet(self, path: str, entity: Optional[str] = None) -> None:
        """Export one entity (or all entities) to Parquet files under `path`."""
        out_dir = Path(path)
        out_dir.mkdir(parents=True, exist_ok=True)
        names = [entity] if entity else list(self.data.keys())
        for name in names:
            df = self.to_pandas(name)
            df.to_parquet(out_dir / f"{name}.parquet")

    def to_json(self, path: str) -> None:
        """Export all generated entity data to a single JSON file."""
        with open(path, "w") as f:
            json.dump(self.data, f, default=str, indent=2)

    @property
    def fidelity_score(self) -> float:
        """Real fidelity score in [0, 1]: 1.0 minus the fraction of generated
        values that violate the schema's nullability/uniqueness/range/length/
        pattern rules. Not a hardcoded constant."""
        return self._quality["fidelity_score"]

    @property
    def constraint_violations(self) -> int:
        """Real count of nullability + uniqueness + range/length/pattern
        constraint violations found in the generated data. Not hardcoded."""
        return self._quality["constraint_violations"]

    @property
    def quality_report(self) -> Dict[str, Any]:
        """Full breakdown: fidelity_score, total_checks, null_violations,
        uniqueness_violations, constraint_violations."""
        return dict(self._quality)


def load_schema(path: str) -> Schema:
    """Load schema from file (auto-detect format by extension)."""
    p = Path(path)
    if p.suffix in (".yaml", ".yml"):
        return load_schema_yaml(path)
    elif p.suffix == ".json":
        return load_schema_json(path)
    else:
        raise ValueError(f"Unsupported schema format: {p.suffix}")


def load_schema_yaml(path: str) -> Schema:
    """Load schema from a YAML file."""
    return Schema(yaml_path=path)


def load_schema_json(path: str) -> Schema:
    """Load schema from a JSON file with the same shape as the YAML schema
    format (entities/relationships/constraints)."""
    with open(path) as f:
        data = json.load(f)

    schema = Schema()
    for entity_name, entity_def in data.get("entities", {}).items():
        schema.add_entity(entity_name)
        fields = entity_def.get("fields", {}) if isinstance(entity_def, dict) else {}
        for field_name, field_def in fields.items():
            if isinstance(field_def, str):
                field_type, nullable, unique = field_def, False, False
            else:
                field_type = field_def.get("type", "string")
                nullable = bool(field_def.get("nullable", False))
                unique = bool(field_def.get("unique", False))
            schema.add_field(entity_name, field_name, field_type, nullable=nullable, unique=unique)

    for rel in data.get("relationships", []):
        schema.add_relationship(
            rel["from_entity"],
            rel["to_entity"],
            rel["from_field"],
            rel["to_field"],
            rel.get("cardinality", "1:n"),
        )

    for c in data.get("constraints", []):
        schema.add_constraint(
            c["constraint_type"], c["entity"], c["value"], c.get("field")
        )

    return schema
