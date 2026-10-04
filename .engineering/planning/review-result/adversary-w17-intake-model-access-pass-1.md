---
format: aep.planning-md/3
id: review-result:adversary-w17-intake-model-access-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w17 adversary, intake story:model-access, pass 1
relations:
- reviews: story:model-access
revision: 1
---
## Adversary pass (single) — intake story:model-access

Tree: dac69da + phase 2. Verdict CONFIRMED; cases 52→63, red 5. New file: `crates/intake-model/tests/adversary_model.rs`.

```findings
- file: crates/intake-model/src/lib.rs
  line: 289
  category: contract-drift
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: a stream closed before a terminal event was Model(Protocol), not Transport
- file: crates/intake-model/src/lib.rs
  line: 112
  category: boundary
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: a relative CODEX_HOME gave a relative login path that llm refuses on every call
- file: crates/intake-model/src/lib.rs
  line: 259
  category: concurrency
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: the kept login refusal was checked before the turn result
- file: crates/intake-model/src/lib.rs
  line: 224
  category: concurrency
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: a login refusal on a spawned task misses the task-local slot
- file: crates/intake-model/src/lib.rs
  line: 263
  category: contract-drift
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: non-object arguments from a recorded model were returned as an answer
- file: crates/intake-model/src/lib.rs
  line: 279
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: mapping every unusable login to MissingCredential survived the phase-2 suite
- file: crates/intake-model/src/lib.rs
  line: 267
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: returning the first of several forced calls survived; MultipleToolCalls was untested
```

| # | finding | decision |
|---|---|---|
| 1 | lib.rs:289 a stream closed before a terminal event is Model(Protocol), not Transport | accept, fix: classify llm's "the stream ended before the response reached a terminal state" Protocol refusal as Transport, in the same pinned-message function as the tool refusals, with a unit test pinning the llm message |
| 2 | lib.rs:112 relative CODEX_HOME gives a relative login path | accept, fix: codex_auth_path refuses a relative CODEX_HOME (and a relative HOME) as MissingCredential with a message naming the variable |
| 3 | lib.rs:259 kept refusal overrides a successful turn | accept, fix: read the slot only when the turn returned Err |
| 4 | lib.rs:224 refusal on a spawned task loses the typed kind | decline: no Model wrapper spawns; document the limit on CodexLogin / call_tool. The coordinator marked the case `a_refusal_on_a_spawned_task_is_still_a_credential_refusal` `#[ignore]` with this reason in adversary_model.rs |
| 5 | lib.rs:263 non-object arguments from a recorded model are an answer | accept, fix: call_tool refuses non-object arguments (same error llm gives over HTTP, or a ModelError variant named for it) |
| 6 | lib.rs:279 Unusable mapped to Missing survived | accept; adversary case kills it; no code change |
| 7 | lib.rs:267 MultipleToolCalls untested | accept; adversary case kills it; no code change |

After the fixes: unit gate 66 tests (1 declined case ignored); integration gate on 5e4c072: 77 tests passed, all steps exit 0.
