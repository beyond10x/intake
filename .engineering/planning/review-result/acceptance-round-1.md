---
format: aep.planning-md/3
id: review-result:acceptance-round-1
kind: review-result
status: active
title: Acceptance critic, round 1
relations:
- reviews: story:model-access
- reviews: story:router-classifier
- reviews: story:case-frontier
- reviews: story:intent-references
- reviews: story:selector-executor
- reviews: story:slice-loop-cli
- reviews: epic:intake-slice
revision: 1
---
needs-revision

story:slice-loop-cli — the acceptance observes two of the five stop reasons the outcome promises (`ApprovalRequired`, `NoLocalExecutor`), so `NothingAdmissible`, `StepBudget` and `Refused` can ship untested and the story still passes — .engineering/planning/story/slice-loop-cli.md:38
story:intent-references — the outcome promises that the references are listed in the run's transcript and given to Loom's selection context, but the acceptance only checks `references(intent)` output, so the handoff that is the story's stated purpose is never observed — .engineering/planning/story/intent-references.md:28
epic:intake-slice — the live-run check names `picked software.change@1`, but the registry names the protocol `software-change` (`protocols/software-change/1.yaml`) and the story acceptance prints `picked software-change@1`, so the done-when string cannot match the output — .engineering/planning/epic/intake-slice.md:48
story:case-frontier — the acceptance runs on the `chg-1842` fixture, which carries its own revisions (`i1`, `R2`), so the outcome's derivation (intent revision is a hash of the intent text, `implementation` is git `HEAD`, other artifacts `r0`) is never observed — .engineering/planning/story/case-frontier.md:33
story:selector-executor — the outcome says `repository.inspect` returns file contents, but the acceptance exercises only `repository.edit`, `tests.run`, `repository.merge` and an out-of-workspace path, so `inspect` is untested — .engineering/planning/story/selector-executor.md:38
story:model-access — the outcome names four typed errors (no key, transport, no tool call, wrong tool) but the acceptance observes only `MissingKey` and `NoToolCall`, so the `transport` and `wrong tool` errors are unobserved — .engineering/planning/story/model-access.md:28

What I read: 7 artifacts (epic:intake-slice and its six stories), all read in full with `aep plan artifact show <id>` and `aep plan artifact kinds`. I also read `aep plan artifact lifecycle story`, `ess/domains/routing.yaml`, and the els `protocols/` directory, `registry.rs` and `fixtures/software-change/chg-1842.fixture.yaml`.

What I could not establish:
- Whether `intake-model` can point its binding at a local HTTP fixture, which model-access's acceptance relies on. That is design, not acceptance, so it is out of my lane and does not set my verdict.
- The multi-clause acceptances (`;` joining several cases under one named test) are not flagged, because each is a single named test that passes or fails as a whole.

```findings
- file: .engineering/planning/story/slice-loop-cli.md
  line: 38
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance observes two of the five stop reasons the outcome promises (ApprovalRequired, NoLocalExecutor), so NothingAdmissible, StepBudget and Refused can ship untested and the story still passes"
- file: .engineering/planning/story/intent-references.md
  line: 28
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the outcome promises the references are listed in the run's transcript and given to Loom's selection context, but the acceptance only checks references(intent) output, so the handoff that is the story's stated purpose is never observed"
- file: .engineering/planning/epic/intake-slice.md
  line: 48
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the live-run check names `picked software.change@1`, but the registry names the protocol software-change and the story acceptance prints `picked software-change@1`, so the done-when string cannot match the output"
- file: .engineering/planning/story/case-frontier.md
  line: 33
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance runs on the chg-1842 fixture, which carries its own revisions (i1, R2), so the outcome's derivation (intent revision is a hash of the intent text, implementation is git HEAD, other artifacts r0) is never observed"
- file: .engineering/planning/story/selector-executor.md
  line: 38
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the outcome says repository.inspect returns file contents, but the acceptance exercises only repository.edit, tests.run, repository.merge and an out-of-workspace path, so inspect is untested"
- file: .engineering/planning/story/model-access.md
  line: 28
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the outcome names four typed errors (no key, transport, no tool call, wrong tool) but the acceptance observes only MissingKey and NoToolCall, so the transport and wrong-tool errors are unobserved"
```
