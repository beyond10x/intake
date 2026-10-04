---
format: aep.planning-md/3
id: review-result:adversary-w17-intake-router-classifier-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w17 adversary, intake story:router-classifier, pass 1
relations:
- reviews: story:router-classifier
revision: 1
---
## Adversary pass (single) — intake story:router-classifier

Tree: a8358ea + phase 2. Verdict: no defect; cases 78→86, red 0. New file: `crates/intake-router/tests/adversary_classify.rs` (8 cases, each killing a mutant the acceptance test missed).

```findings
- file: crates/intake-router/tests/classify.rs
  line: 173
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: the acceptance test stayed green under nine mutants of classify.rs; adversary_classify.rs catches each
- file: crates/intake-router/src/classify.rs
  line: 196
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: llm sends the pick tool with strict false, so the post-call registry check is the only defence against a steered pick, and it holds
```

| # | decision |
|---|---|
| 1 | accept; closed by the adversary cases |
| 2 | accept as the design: the registry check is the defence; no change |

Integration gate on 7e20e76: 99 tests passed, 2 ignored; all steps exit 0.
