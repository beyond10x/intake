---
format: aep.planning-md/3
id: story:slice-loop-cli
kind: story
status: draft
title: Run the slice from the command line until it is blocked
relations:
- decomposes: epic:intake-slice
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:router-classifier
- depends_on: story:intent-references
- depends_on: story:case-frontier
- depends_on: story:selector-executor
scope:
- confidence: cited
  path: crates/intake-cli/Cargo.toml
- confidence: cited
  path: crates/intake-cli/src/main.rs
- confidence: cited
  path: crates/intake-cli/tests/slice_run.rs
- confidence: cited
  path: crates/intake-slice/src/lib.rs
- confidence: cited
  path: crates/intake-slice/src/run.rs
revision: 5
---
## Outcome

`b10x-intake run --workspace <dir> [--test-cmd <cmd>] [--max-steps N] [--model <id>] [--classifier-model
<id>] [--threshold <x>] "<intent>"` (clap derive) runs the slice the way Commission's runtime loop will:

1. Extract the intent's references.
2. Classify the intent.
3. Open the governed case.
4. Loop: `Governor::frontier`, then `Loom::run` (the `AgentExecutor`), then the local executor
   performs the proposed action, then the verifier submits evidence, then `Governor::completion`.

The run ends with a `SliceRun` stop reason:

| Stop reason | When |
|---|---|
| `ApprovalRequired` | the only useful action needs authority |
| `NothingAdmissible` | no action is admissible |
| `StepBudget` | `--max-steps` is used up |
| `NoLocalExecutor` | the picked protocol is not `software.change/1` |
| `Refused` | the router refuses (outside the registry, or unsure) |

It prints the references, the pick, each step (action, arguments, effect, evidence) and the stop
reason. The selector and argument generator use `codex_model(--model)`; the classifier uses
`codex_model(--classifier-model)`; both default to `gpt-5.6-sol`, so a smaller classifier can be
swapped in alone.

## Acceptance

`the_slice_stops_for_each_reason`: against a fixture git repository with one failing test and
recorded model responses, `b10x-intake run` prints the references it extracted, then:

| Case | Output |
|---|---|
| main path | `picked software-change@1`, one `repository.edit`, one `tests.run` with `test_result pass`, and `stopped: ApprovalRequired (repository.merge)` |
| a step budget of 1 | `stopped: StepBudget` |
| a recorded pick of `incident-response@1` | `stopped: NoLocalExecutor` after its first frontier |
| a recorded confidence below the threshold | `stopped: Refused (unsure)` |
| a recorded frontier with nothing admissible | `stopped: NothingAdmissible` |

## ESS first

`intake.routing.SliceRun` and `StopReason` are declared. The first commit is the named test, red
because the binary does not exist.

## Live qualification

After this story, one live run on a scratch repository under the operator's Codex subscription is
recorded as evidence on `epic:intake-slice`.
