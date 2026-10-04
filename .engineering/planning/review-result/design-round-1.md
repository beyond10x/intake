---
format: aep.planning-md/3
id: review-result:design-round-1
kind: review-result
status: active
title: Design critic, round 1
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

story:router-classifier — `classify(intent, model)` is tested against a fake `llm_core::Model` and forces its own `pick_protocol` tool, while story:model-access returns only a `serde_json::Value` from `call_tool` and nothing in the set turns the haiku binding into a `llm_core::Model` for the router. Either the body takes the `call_tool` seam (adding a `depends_on story:model-access` edge) or model-access exposes a `Model` and the cli wires it — .engineering/planning/story/router-classifier.md:24

story:intent-references — the outcome claims the slice lists references in the transcript and hands them to Loom's selection context, but its scope is `crates/intake-router` only. That half is described by story:selector-executor and story:slice-loop-cli, so the same behaviour sits in three bodies. Cut the sentence to "`classify` returns the references beside the pick" and leave the hand-off to those two — .engineering/planning/story/intent-references.md:28

story:selector-executor — the selector "is given … its references", which are the `ExtractedReference` values story:intent-references produces, and no edge records that order (only model-access and case-frontier are declared). Add `depends_on story:intent-references` — .engineering/planning/story/selector-executor.md:30; `aep plan artifact graph`

What I read: 7 artifacts (the epic and six stories) with `aep plan artifact show`, plus `relations`, `graph` and `validate`. I walked all 28 edges in the graph, including those to `vision:O2` and `vision:governed-autonomy`. The graph is acyclic and is not a single chain: model-access, router-classifier and case-frontier are roots. `validate` reports valid. I also read `ess/domains/routing.yaml`, `llm-core/src/port.rs` and `loom/src/selection.rs`.

What I could not establish:
- Whether `ExtractedReference` and `ProtocolPick` are Rust types owned by `intake-router`, so I could not tell whether `intake-slice` also needs an edge for those types. That would be a parallel-safety question.
- Whether `intake-router::classify` returning references beside the pick breaks the return type in story:router-classifier's acceptance. That is out of my lane (parallel safety / acceptance); the `depends_on` edge already exists.

```findings
- file: .engineering/planning/story/router-classifier.md
  line: 24
  category: design
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "classify(intent, model) is tested against a fake llm_core::Model and forces its own tool, while story:model-access returns only a serde_json::Value from call_tool and nothing in the set yields a llm_core::Model for the haiku classifier, so the seam between the two stories is unowned and no depends_on records it"
- file: .engineering/planning/story/intent-references.md
  line: 28
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the outcome claims the slice lists references in the transcript and hands them to Loom's selection context, which its intake-router-only scope cannot deliver and which story:selector-executor and story:slice-loop-cli also describe, so the hand-off is split across three bodies"
- file: .engineering/planning/story/selector-executor.md
  line: 30
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the selector is given the references that story:intent-references produces, and no depends_on edge to story:intent-references records that order"
```
