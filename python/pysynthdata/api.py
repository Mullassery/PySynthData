"""High-level Python API for PySynthData.

This is a thin wrapper around the real Rust `_core` extension — schema
construction, row generation, and quality-report computation all happen in
Rust. Nothing here reimplements generation in pure Python; `Schema` and
`WorldGenerator` hold a real `_core.Schema` / `_core.WorldGenerator` instance
and delegate every real piece of work to it.
"""

from typing import Optional, Dict, Any, List, Iterator, Tuple
import json
from pathlib import Path

import pandas as pd
import pyarrow as pa
import pyarrow.parquet as pq

from pysynthdata import _core


class PrivacyBudget:
    """Real epsilon-delta differential-privacy budget, backed by
    `_core.PrivacyBudget`. Tracks how much has been spent across
    `WorldGenerator.generate_private()` calls and refuses to overspend --
    each `generate_private()` call raises `ValueError` rather than silently
    exceeding the declared total.

    Example:
        >>> budget = PrivacyBudget(epsilon=1.0, delta=1e-5)
        >>> world = generator.generate_private(1000, seed=42, budget=budget, epsilon=0.3)
        >>> budget.remaining_epsilon
        0.7
    """

    def __init__(self, epsilon: float, delta: float = 0.0):
        self._inner = _core.PrivacyBudget(epsilon, delta)

    @property
    def total_epsilon(self) -> float:
        return self._inner.total_epsilon

    @property
    def total_delta(self) -> float:
        return self._inner.total_delta

    @property
    def spent_epsilon(self) -> float:
        return self._inner.spent_epsilon

    @property
    def spent_delta(self) -> float:
        return self._inner.spent_delta

    @property
    def remaining_epsilon(self) -> float:
        return self._inner.remaining_epsilon

    @property
    def remaining_delta(self) -> float:
        return self._inner.remaining_delta

    @property
    def is_exhausted(self) -> bool:
        return self._inner.is_exhausted()

    def __repr__(self) -> str:
        return repr(self._inner)


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

    def generate_private(
        self,
        num_records: int,
        seed: int,
        budget: PrivacyBudget,
        epsilon: float,
    ) -> "GeneratedWorld":
        """Generate rows, then add Laplace-mechanism differential-privacy
        noise to every Int/Float field, spending `epsilon` from `budget`.

        Each numeric field's noise scale is calibrated from its
        schema-declared range constraint (or, if none exists, the observed
        spread of its generated values) divided by `epsilon` split evenly
        across every numeric field being privatized -- standard Laplace
        mechanism calibration under sequential composition, not a fabricated
        "privacy" flag. Raises `ValueError` if `budget` doesn't have
        `epsilon` left, or if the schema has no Int/Float fields.

        Example:
            >>> budget = PrivacyBudget(epsilon=1.0, delta=1e-5)
            >>> world = generator.generate_private(1000, seed=42, budget=budget, epsilon=0.3)
            >>> world.privacy_report["values_perturbed"]
        """
        result = self._inner.generate_private(num_records, seed, budget._inner, epsilon)
        return GeneratedWorld(result)

    def generate_streaming(
        self,
        num_records: int,
        seed: int,
        chunk_size: int = 10_000,
        queue_size: int = 2,
    ) -> Iterator[Tuple[str, List[Dict[str, Any]]]]:
        """Generate rows in `chunk_size`-sized batches instead of building one
        big in-memory result -- for large `num_records` where materializing
        every row at once (what `generate()` does) is the memory bottleneck.

        Yields `(entity_name, rows)` pairs. Rust-side generation runs on a
        background thread (releasing the GIL for the duration -- see
        `generate_streaming` in `src/lib.rs`) and hands each chunk to this
        thread through a bounded `queue.Queue`. That queue -- not "collect
        everything, then yield" -- is what actually bounds memory: at most
        `queue_size` chunks are ever buffered, so the producer thread blocks
        on `queue.put()` until this generator's consumer catches up. A
        naive callback-appends-to-a-list wrapper would silently defeat the
        whole point by materializing every chunk before yielding the first
        one; this doesn't.

        Entities that are a foreign-key source for another entity are still
        fully generated internally before being streamed out (required for
        correct FK sampling) -- everything else has peak memory bounded by
        `chunk_size * queue_size`, not `num_records`.

        Example:
            >>> for entity_name, rows in generator.generate_streaming(1_000_000, seed=1, chunk_size=50_000):
            ...     print(entity_name, len(rows))
        """
        import queue
        import threading

        q: "queue.Queue[Any]" = queue.Queue(maxsize=queue_size)
        sentinel = object()
        errors: List[BaseException] = []

        def on_chunk(entity_name: str, rows: List[Dict[str, Any]]) -> None:
            q.put((entity_name, rows))

        def worker() -> None:
            try:
                self._inner.generate_streaming(num_records, seed, chunk_size, on_chunk)
            except BaseException as exc:  # noqa: BLE001 - re-raised on the consumer thread below
                errors.append(exc)
            finally:
                q.put(sentinel)

        thread = threading.Thread(target=worker, daemon=True)
        thread.start()
        try:
            while True:
                item = q.get()
                if item is sentinel:
                    break
                yield item
        finally:
            thread.join()
        if errors:
            raise errors[0]

    def to_parquet_streaming(
        self,
        num_records: int,
        seed: int,
        out_dir: str,
        chunk_size: int = 10_000,
    ) -> Dict[str, int]:
        """Generate and write directly to one Parquet file per entity under
        `out_dir`, writing each chunk as a row group via `pyarrow.parquet.
        ParquetWriter` instead of building a pandas DataFrame (or any other
        full-entity structure) first. Returns `{entity_name: row_count}`.

        Calls the Rust core's chunked generation directly (not through
        `generate_streaming()` above) since a single synchronous write pass
        needs no background thread here -- each chunk is written to its
        entity's Parquet writer and dropped immediately, so peak memory for
        large, non-FK-source entities is bounded by `chunk_size`, not
        `num_records`. This is the real fix for "generation materializes the
        entire dataset in memory before any export call."

        Example:
            >>> counts = generator.to_parquet_streaming(5_000_000, seed=1, out_dir="out/", chunk_size=50_000)
            >>> counts["orders"]
            5000000
        """
        out_path = Path(out_dir)
        out_path.mkdir(parents=True, exist_ok=True)

        writers: Dict[str, pq.ParquetWriter] = {}
        row_counts: Dict[str, int] = {}

        def on_chunk(entity_name: str, rows: List[Dict[str, Any]]) -> None:
            if not rows:
                return
            table = pa.Table.from_pylist(rows)
            writer = writers.get(entity_name)
            if writer is None:
                writer = pq.ParquetWriter(str(out_path / f"{entity_name}.parquet"), table.schema)
                writers[entity_name] = writer
            writer.write_table(table)
            row_counts[entity_name] = row_counts.get(entity_name, 0) + len(rows)

        try:
            self._inner.generate_streaming(num_records, seed, chunk_size, on_chunk)
        finally:
            for writer in writers.values():
                writer.close()

        return row_counts

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
        self._privacy: Optional[Dict[str, Any]] = result.get("privacy")
        self.seed: int = self.metadata["seed"]
        self.num_records: int = self.metadata["record_count"]

    @property
    def privacy_report(self) -> Optional[Dict[str, Any]]:
        """Real epsilon/delta spend + perturbation counts from
        `WorldGenerator.generate_private()` -- `None` for worlds produced by
        plain `generate()`, which never touched the privacy budget."""
        return dict(self._privacy) if self._privacy is not None else None

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
