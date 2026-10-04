---
format: aep.planning-md/3
id: review-result:design-round-2
kind: review-result
status: active
title: Design critic, round 2
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

story:case-frontier — its outcome says "a revision update bumps the case revision" but names no entry point on the governor, and the `Governor` trait has none (`current_revision`, `frontier`, `completion` only). story:selector-executor's `repository.edit` calls such an entry point ("reports the new `implementation` revision to the governor"), so each body describes one half of the revision seam. The body should name the update method (case, artifact, new revision) as a deliverable of `intake-governor` and have the acceptance exercise it. — .engineering/planning/story/case-frontier.md:32 (other half: .engineering/planning/story/selector-executor.md:41)

What I read: 8 artifacts (the epic, the 5 stories and the prior `review-result:design-round-1`) with `aep plan artifact show`, plus `relations`, `graph` and `validate`. I walked all edges in the graph, including those to the visions. The graph is acyclic and is not a chain. The roots are model-access, case-frontier and intent-references. `validate` reports valid. I also read commission `ports/governor.rs`, `ports/evidence.rs` and `ports/executor.rs`, and loom `lib.rs` (`AgentExecutor for Loom`).

Round 1 is closed:
- router-classifier now takes a `&dyn Model` and has `depends_on` model-access.
- intent-references is now its own crate and no longer claims the hand-off.
- selector-executor now has `depends_on` intent-references.

What I could not establish (outside my lane, so they do not set the verdict):
- `Loom::run` takes a `Commission<Assigned>`, and no body says who builds and assigns it in the slice-loop-cli loop. That is for scope or acceptance.
- `submit_evidence` refuses evidence that names no observation, and the governor implements `EvidencePort` only. No body says how the executor's observation reaches the governor (`ObservationPort`) or whether the verifier is an `EvidenceAdapter`. That is for acceptance or parallel-safety.
- The transcript type that the selector reads and slice-loop-cli's `run.rs` builds has no named owner. I could not tell whether that is a seam.

```findings
- file: .engineering/planning/story/case-frontier.md
  line: 32
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the outcome says a revision update bumps the case revision but names no governor entry point for it (the Governor trait has none), while story:selector-executor's repository.edit reports the new implementation revision to the governor, so the revision-update seam is described by two bodies and owned by neither"
```
