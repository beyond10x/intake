---
format: aep.planning-md/3
id: story:case-frontier
kind: story
status: implemented
title: Open the slice's case through the governor
refs:
- provider: governor
  reference: story:canon-governor
relations:
- decomposes: epic:intake-slice
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: crates/intake-slice/Cargo.toml
- confidence: cited
  path: crates/intake-slice/src/case.rs
- confidence: cited
  path: crates/intake-slice/src/lib.rs
- confidence: cited
  path: crates/intake-slice/tests/slice_case.rs
revision: 14
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T15:17:43Z", actor: "human:timo", revision: 12, decided_on: {"recorded":{"review_outcome":3}}}
- {from: "proposed", to: "active", at: "2026-10-04T15:17:43Z", actor: "human:timo", revision: 13, decided_on: {"recorded":{"review_outcome":3}}}
- {from: "active", to: "implemented", at: "2026-10-04T17:19:10Z", actor: "human:timo", revision: 14, decided_on: {"recorded":{"test_result":1,"review_outcome":5,"verification":1}}}
---
## Outcome

The slice opens its case through the governor in `beyond10x/governor` (story `canon-governor`
there). It does not evaluate Canon itself. `intake-slice::case::open(pick, intent, workspace)`
derives the artifact revisions a `software.change/1` case starts with:

- `intent`: a hash of the intent text;
- `implementation`: the workspace's git `HEAD`;
- every other declared artifact: `r0`.

It then calls the governor's `open(protocol, revisions)` and returns the case id. After a
`repository.edit`, the slice reports the new `HEAD` through the governor's
`update_revision(case, implementation, revision)`.

## Acceptance

`the_slice_case_starts_where_the_workspace_is`: from an intent and a fixture git repository, the
opened case holds the intent hash, `HEAD` and `r0` for every other artifact. Its first frontier, as
the governor issues it, makes `repository.edit` and `tests.run` admissible and `repository.merge`
blocked with the reason `implementation.verified`. A commit in the fixture repository, reported
through `update_revision`, raises the case revision by one.

## ESS first

None in `intake.routing`. The first commit is the named test, red because `case::open` does not
exist.

## Depends on

beyond10x/governor story `canon-governor` (implemented in wave 2026-10-04-w16), pinned at the exact
revision `3d028b8`. The governor has no release process yet; a tag replaces the revision when it
does. Canon comes through the governor and ELS by the same reference (`branch = "main"`, pinned by
`Cargo.lock`), so one Canon builds.
