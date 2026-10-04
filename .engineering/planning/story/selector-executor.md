---
format: aep.planning-md/3
id: story:selector-executor
kind: story
status: implemented
title: Choose actions with a model through Loom and perform them locally in the workspace
relations:
- decomposes: epic:intake-slice
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:model-access
- depends_on: story:case-frontier
- depends_on: story:intent-references
scope:
- confidence: cited
  path: crates/intake-slice/Cargo.toml
- confidence: cited
  path: crates/intake-slice/src/executor.rs
- confidence: cited
  path: crates/intake-slice/src/lib.rs
- confidence: cited
  path: crates/intake-slice/src/selector.rs
- confidence: cited
  path: crates/intake-slice/src/verifier.rs
- confidence: cited
  path: crates/intake-slice/tests/selector_executor.rs
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T17:22:51Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":2}}}
- {from: "proposed", to: "active", at: "2026-10-04T17:22:51Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"review_outcome":2}}}
- {from: "active", to: "implemented", at: "2026-10-04T18:01:54Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"test_result":1,"review_outcome":4,"verification":1}}}
---
## Outcome

Loom chooses, the slice performs, and only a trusted verifier turns what happened into evidence
(Atlas ADR 0074: nothing turns executor output into evidence).

- **Selector and argument generator.** `intake-slice` implements Loom's `ActionSelector` and
  `ArgumentGenerator` over a `&dyn llm_core::Model`.
  - The selector is given the catalogue entries, the intent, its `ExtractedReference`s and the
    transcript so far. It must pick one listed action; Loom refuses any other.
  - The generator writes that action's arguments.
- **Local executor.** It performs a proposed `software.change/1` action inside the workspace only:
  - `repository.inspect` returns file contents;
  - `repository.edit` writes the given files and commits them, and reports the new `implementation`
    revision to the governor;
  - `tests.run` runs the configured test command.

  `repository.merge` and every other action are never executed.
- **Test-result verifier.** The executor's report is an observation. A test-result verifier, the
  trusted integration and never the model, reads the command's exit status and submits
  `test_result` (`pass`/`fail`) about the current `implementation` revision through
  `submit_evidence`.

## Acceptance

`a_failing_test_is_edited_and_then_passes`: the setup is a fixture git repository with one failing
test and recorded model responses that choose `repository.inspect`, then `repository.edit` (a file
that fixes the test), then `tests.run`. The test checks that:

- the selector's context contains the intent's references;
- inspect returns the file;
- the edit commits a new revision;
- the verifier submits `test_result pass` about that revision;
- a recorded choice of `repository.merge` is not executed;
- an edit path outside the workspace is refused;
- a recorded model answer that claims a passing test produces no evidence.

## ESS first

None in `intake.routing`. The first commit is the named test, red because the selector, generator,
executor and verifier do not exist.

## Reuses

- `loom/crates/loom/src/selection.rs:43` (`ActionSelector`)
- `loom/crates/loom/src/arguments.rs:32` (`ArgumentGenerator`)
- `loom/crates/loom/src/lib.rs:182` (`Loom::run`)
- `commission/crates/commission/src/ports/evidence.rs` (`submit_evidence`)
