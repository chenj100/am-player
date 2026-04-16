# Contract: Source Adapter Boundary

## Purpose

Provider-specific logic must sit behind a stable adapter interface so policy
changes, breakage, or provider removal do not destabilize the rest of the app.

## Required Operations

### `search(query) -> TrackCandidate[]`

Returns normalized search candidates with:
- source identifiers
- display metadata
- availability state
- provider-specific raw fields retained only in adapter internals

### `inspect(candidate) -> SourceInspection`

Returns:
- whether save is currently allowed by product policy
- whether local save is technically available
- metadata hints
- estimated size if available

### `prepare_save(candidate, destination) -> SavePlan`

Returns:
- policy result
- target filename suggestion
- temporary work requirements
- whether Android-native execution is required

### `execute_save(plan) -> SaveResult`

Returns:
- completion status
- local file output if created
- export artifact if created
- provider error classification

## Adapter Rules

- Adapter code must not leak provider-specific raw responses into UI contracts.
- Provider disablement must happen through configuration and policy checks, not
  scattered conditionals in the UI.
- Adapter failures must classify into:
  `policy_blocked`, `network`, `auth`, `source_changed`, `rate_limited`,
  `unavailable`, `unknown`

## Initial Adapter Strategy

- `youtube`: metadata/search-capable adapter, but download capability must be
  treated as policy-constrained and independently disableable
- `bilibili`: adapter may rely on provider-specific metadata discovery patterns,
  with search and metadata treated separately from save execution
