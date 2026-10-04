---
format: aep.planning-md/3
id: review-result:scope-round-1
kind: review-result
status: active
title: Scope critic, round 1
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

- story:slice-loop-cli — the epic names which model each role uses ("classifier `claude-haiku-4-5-20251001`, selector, argument generator and edit tool `claude-sonnet-5-5`"). No story claims the classifier's haiku id, and none says what "edit tool" means. `classify` only takes an unspecified `model`, selector-executor names sonnet for the selector and generator only, and nothing says who binds haiku to the router. The CLI story, which wires the whole run, should state the per-role model ids, or selector-executor should say whether the "edit tool" is a separate model call — `.engineering/planning/epic/intake-slice.md:41`

**What I read.** 7 artifacts: the epic, 6 stories, plus `ess/domains/routing.yaml`. Commands: `aep plan artifact show epic:intake-slice`, `aep plan artifact show story:<each of the six>`, `aep plan artifact graph`, `cat ess/domains/routing.yaml`, and greps of the stories for `haiku|sonnet|edit tool|max-steps|threshold`. I extracted 12 promises from the epic and traced 11 to a story; the model-binding promise above is the one I could not trace. The epic's `UNMAPPED:` marker on reference kinds is named in the intent-references story, so it is not a gap.

**What I could not establish.**
- The operator's second request says the extracted references should let the next node "have it preloaded". The intent-references story names this as not in scope ("Fetching what a reference points at… a later story"). The epic itself only promises "extracts its key references", so I treated this as a recorded exclusion and not a gap. It is worth confirming with the operator that the later story exists somewhere.
- Operator request 2 says the classifier "should also extract". The story extracts with deterministic patterns and no model call, which is a design call and not a coverage defect.
- `story:intent-references` changes `classify`'s return type while `story:router-classifier` owns `classify`, and both touch `crates/intake-router/src/lib.rs`. That is out of my lane; it belongs to the design and parallel-safety critics.
- The slice-loop-cli acceptance covers `ApprovalRequired` and `NoLocalExecutor` but not `NothingAdmissible`, `StepBudget` or `Refused`. That is out of my lane; it belongs to the acceptance critic.

```findings
- file: .engineering/planning/epic/intake-slice.md
  line: 41
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the epic promises classifier model claude-haiku-4-5-20251001 and an \"edit tool\" on claude-sonnet-5-5, and no story claims the haiku binding for the router or says what the edit-tool model call is (slice-loop-cli would most naturally claim the wiring)"
```
