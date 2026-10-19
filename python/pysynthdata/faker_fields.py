"""Optional Faker-backed field enrichment.

The Rust generator owns relational integrity/constraints. Faker adds realistic
locale-aware values (names, emails, addresses) after generation without
replacing the schema engine. Missing Faker raises an actionable error.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Iterable


class FakerUnavailable(RuntimeError):
    """Raised when Faker is not installed."""


@dataclass(frozen=True)
class FakerFieldSpec:
    entity: str
    field: str
    provider: str


def parse_spec(value: str) -> FakerFieldSpec:
    """Parse ``Entity.field=provider``."""
    target, sep, provider = value.partition("=")
    entity, dot, field = target.partition(".")
    if not sep or not dot or not entity or not field or not provider:
        raise ValueError(f"expected Entity.field=provider, got {value!r}")
    return FakerFieldSpec(entity=entity, field=field, provider=provider)


def apply_faker_fields(
    world: Any,
    specs: Iterable[FakerFieldSpec],
    *,
    locale: str = "en_US",
    seed: int | None = None,
) -> Any:
    try:
        from faker import Faker
    except ImportError as exc:
        raise FakerUnavailable(
            "Faker is not installed. Install the optional extra with "
            '`pip install "pysynthdata[faker]"`.'
        ) from exc

    fake = Faker(locale)
    if seed is not None:
        fake.seed_instance(seed)
    for spec in specs:
        if spec.entity not in world.data:
            raise KeyError(f"entity {spec.entity!r} not found in generated world")
        provider = getattr(fake, spec.provider, None)
        if provider is None or not callable(provider):
            raise ValueError(f"Faker has no provider {spec.provider!r}")
        for row in world.data[spec.entity]:
            row[spec.field] = provider()
    return world


__all__ = ["FakerFieldSpec", "FakerUnavailable", "apply_faker_fields", "parse_spec"]
