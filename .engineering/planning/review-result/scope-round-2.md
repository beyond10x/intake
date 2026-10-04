---
format: aep.planning-md/3
id: review-result:scope-round-2
kind: review-result
status: active
title: Scope critic, round 2
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

- story:slice-loop-cli — the epic promises "the classifier's model is configurable so a smaller one can replace it", but the CLI has one `--model` that sets every role, so the classifier cannot be swapped for a smaller model without also swapping the selector and argument generator; add a classifier-specific option, or state that the promise is narrowed — `.engineering/planning/epic/intake-slice.md:46`
- story:model-access — the title says "a Claude model" while the epic and the story's own outcome bind the Codex subscription (operator request 3: not an Anthropic route), so the title claims a different outcome than the body delivers; retitle it to the Codex route — `.engineering/planning/story/model-access.md:6`

**What I read.** 8 artifacts (the epic, the 6 stories, `review-result:scope-round-1`) and the store's epic and story files for line numbers. Commands: `aep plan artifact show` on each, `aep plan artifact graph`, and greps for `configurable|--model|Every model role|title`. I wrote the epic's promise list before reading the stories: 15 promises extracted, 13 traced to a story, 2 not fully traced (the classifier-model finding above and the stale title, which concerns a traced promise).
- The round-1 finding (haiku binding and the "edit tool") is fixed. The epic now says every role uses `gpt-5.6-sol` and the "edit tool" is gone (`.engineering/planning/epic/intake-slice.md:45`; `.engineering/planning/story/slice-loop-cli.md:50`).
- Operator request 4 is covered. `story:case-frontier` implements Commission's `Governor` and `EvidencePort` over Canon, and the selector and CLI stories drive it.
- Operator request 3 is covered. `story:model-access` pins the llm `openai-access` release and defers the Responses client and Codex credential to llm.
- The epic's `UNMAPPED:` reference-kinds marker is named in `story:intent-references`. That story's "Not in scope" on fetching what a reference points at matches an epic that promises only extraction, so I counted it as a recorded exclusion and not a gap.

**What I could not establish.**
- The epic body never names Commission's Governor port or an `intake-governor` crate. It says "Canon evaluates" and "opens a case", and the Governor appears only in "Why" and as the thing that replaces `intake-slice` in "Bounds". `story:case-frontier` is traceable to "Canon evaluates" and to operator request 4. I did not treat it as reach beyond the parent, but the epic could say the governor is in this set.
- Out of my lane: `story:selector-executor` and `story:slice-loop-cli` both touch `crates/intake-slice/src/lib.rs`, which belongs to the parallel-safety critic. They are ordered by `depends_on`.

```findings
- file: .engineering/planning/epic/intake-slice.md
  line: 46
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the epic promises \"the classifier's model is configurable so a smaller one can replace it\", but story:slice-loop-cli has one --model that sets every role, so the classifier cannot be swapped alone; the CLI story should add a classifier-specific option or state the narrowing"
- file: .engineering/planning/story/model-access.md
  line: 6
  category: scope
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the title says \"a Claude model\" while the epic and the story's own outcome bind the operator's Codex subscription, so the title claims a different outcome than the body delivers"
```
