<!--
Sync Impact Report
- Version change: template -> 1.0.0
- Modified principles:
  - Principle 1 placeholder -> I. Code Quality Is a Release Gate
  - Principle 2 placeholder -> II. Tests Prove Behavior
  - Principle 3 placeholder -> III. User Experience Must Stay Consistent
  - Principle 4 placeholder -> IV. Keep Changes Small, Reviewable, and Traceable
  - Principle 5 placeholder -> V. Automation Enforces the Standard
- Added sections:
  - Delivery Standards
  - Review and Workflow
- Removed sections:
  - None
- Templates requiring updates:
  - ✅ updated .specify/templates/plan-template.md
  - ✅ updated .specify/templates/spec-template.md
  - ✅ updated .specify/templates/tasks-template.md
  - ✅ no update needed .specify/templates/agent-file-template.md
  - ✅ no update needed .specify/templates/commands/*.md (directory not present)
- Follow-up TODOs:
  - None
-->
# AM Player Constitution

## Core Principles

### I. Code Quality Is a Release Gate
All production changes MUST be readable, intentionally structured, and small
enough to review with confidence. Code MUST favor clear naming, simple control
flow, explicit error handling, and localized complexity over clever shortcuts.
Lint, formatting, and static analysis rules configured by the repository MUST
pass before merge. Temporary workarounds, dead code, and silent fallback logic
MUST be removed or justified in the relevant specification or plan because they
increase maintenance cost and mask defects.

### II. Tests Prove Behavior
Every behavior change MUST include tests that would fail without the change and
pass with it. New logic requires unit coverage; cross-boundary flows, state
transitions, and bug fixes require integration or regression coverage at the
level where the risk exists. A task plan is incomplete if it omits the test work
needed to prove the feature. If a change cannot be tested automatically, the
specification MUST document why and define the manual validation steps and
acceptance evidence required before release.

### III. User Experience Must Stay Consistent
User-facing changes MUST preserve a coherent experience across screens, states,
copy, inputs, and feedback patterns. New UI work MUST reuse established product
language unless the specification explicitly approves a new pattern. Empty,
loading, error, and success states MUST be designed as first-class behavior, not
left implicit. Accessibility, responsiveness, and interaction clarity MUST be
considered part of feature completeness because inconsistent experiences create
user errors and support burden even when the code is technically correct.

### IV. Keep Changes Small, Reviewable, and Traceable
Each feature specification, plan, and task list MUST map cleanly from user
outcome to implementation work. Tasks MUST identify concrete file paths and
dependencies so changes can be implemented and reviewed independently. Large
multi-purpose edits that combine unrelated refactors with feature delivery MUST
be split unless the plan documents why a combined change is necessary. This
traceability requirement exists to keep defect isolation, review quality, and
rollback options practical.

### V. Automation Enforces the Standard
Build, test, lint, and validation steps MUST be executable in a repeatable way
from the repository so quality does not depend on individual memory or manual
interpretation. Plans and tasks MUST identify the commands or checks needed to
verify compliance. When a gap in automation is discovered, the work SHOULD add
or schedule the missing check; bypassing automation is allowed only when the
specification records the reason, the risk, and the compensating review steps.

## Delivery Standards

Every implementation plan MUST include a constitution check that verifies code
quality gates, required automated tests, and user experience consistency for the
feature. Every specification MUST define measurable outcomes, edge cases, and
user-facing behavior expectations, including state handling where relevant.
Every task list MUST include the validation work needed to prove the feature,
and MUST not mark testing as optional when behavior changes are being delivered.

## Review and Workflow

Work MUST progress from specification to plan to tasks with enough detail for an
independent reviewer to understand scope, risk, and acceptance criteria. Reviews
MUST reject changes that weaken consistency, omit validation, or introduce
unjustified complexity. Before merge, the implementation evidence MUST show:
relevant tests passed, quality checks passed, and user-facing changes were
verified against the specification. Follow-up cleanup MAY be deferred only when
captured explicitly as a tracked task with clear ownership.

## Governance

This constitution overrides conflicting local habits and informal process notes.
Amendments require updating this document together with any affected Spec Kit
templates in the same change. Versioning follows semantic versioning for
governance: MAJOR for incompatible principle changes or removals, MINOR for new
principles or materially expanded guidance, and PATCH for clarifications that do
not change expected behavior. Compliance review is mandatory at plan creation,
task generation, code review, and pre-release validation. Any exception MUST be
documented in the relevant spec or plan with rationale, scope, and the date of
approval.

**Version**: 1.0.0 | **Ratified**: 2026-04-16 | **Last Amended**: 2026-04-16
