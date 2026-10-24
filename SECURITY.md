# Security Policy

## Supported versions

This project is pre-1.0 (`0.5.x`). Only the latest published version on
[PyPI](https://pypi.org/project/pysynthdata/) receives fixes. There is no
LTS branch and no backport policy.

## Reporting a vulnerability

Do not open a public GitHub issue for a security vulnerability. Instead,
email **mullassery@gmail.com** with:

- A description of the issue and its potential impact.
- Steps to reproduce (a minimal schema + code snippet is ideal, given this
  is a synthetic-data-generation library).
- The version/commit you tested against.

This is a single-maintainer open-source project with no dedicated security
team and no formal SLA. Best-effort response time is a few days; there is no
guaranteed patch timeline. If you don't hear back within two weeks, it's fair
to assume the report needs a follow-up, not that it's being ignored.

## Known limitations relevant to security/privacy

Documented plainly (not aspirationally) because this is a data-generation
library and people may reach for it thinking it does more than it does:

- **`PrivacyBudget` / `generate_private()` (`src/privacy.rs`) is real
  differential privacy** (Laplace mechanism, epsilon-delta budget accounting
  with sequential composition) — but only basic/sequential composition is
  implemented. There is no Rényi DP accountant, so repeated queries against
  the same budget are more conservative (spend faster) than a tighter
  accountant would allow. This is a real gap, not a bug, and is tracked in
  `docs/internal/ROADMAP_HONEST.md`.
- **This package makes no compliance claims.** `check_privacy_compliance`,
  `detect_pii_exposure`, `anonymize_dataset` (k-anonymity/l-diversity/
  t-closeness), and `check_fairness_bias` all explicitly return
  `not_implemented` — see `README.md`'s "What's intentionally not here"
  section. An earlier version of this package had `GDPRCompliance`/
  `HIPAACompliance`/`SOC2Compliance` classes that always returned "compliant"
  regardless of input; these were deleted as actively misleading, not kept
  as a stub. If you need certified GDPR/HIPAA/SOC2/PCI-DSS tooling, this
  package does not provide it, full stop.
- **Generated data is not guaranteed free of information leakage from the
  schema/example values you provide.** If you hand-write constraint ranges
  or enum values derived from real sensitive data (e.g., real customer IDs
  as examples), that information can surface in generated rows unless you
  use `generate_private()` with a real epsilon budget on the affected
  numeric fields. Non-numeric fields (strings, enums, UUIDs) have no
  differential-privacy protection at all — only `Int`/`Float` fields get
  Laplace noise.
- **`infer-schema` (`python/pysynthdata/db_schema.py`) connects to a live
  database using credentials you supply on the command line
  (`--db-url`).** Database URLs containing passwords will appear in your
  shell history and process list (`ps`) unless you take precautions (e.g.,
  environment variables, a URL-less auth mechanism your DB driver supports).
  This is standard SQLAlchemy connection-string behavior, not something this
  tool adds on top, but it's worth calling out explicitly.
