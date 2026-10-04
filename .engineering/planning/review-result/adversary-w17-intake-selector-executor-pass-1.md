---
format: aep.planning-md/3
id: review-result:adversary-w17-intake-selector-executor-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w17 adversary, intake story:selector-executor, pass 1
relations:
- reviews: story:selector-executor
revision: 1
---
## Adversary pass (single) — intake story:selector-executor

Tree: 7bb46bc + phase 2. Verdict NEEDS-CHANGE; red 5. New file: `crates/intake-slice/tests/adversary_selector_executor.rs`.

```findings
- file: crates/intake-slice/src/executor.rs
  line: 377
  category: acceptance
  severity: blocker
  verdict: NEEDS-CHANGE
  origin: introduced
  message: the clean-tree check ignored ignored files, so a failed edit leftover yielded test_result pass about a HEAD without it
- file: crates/intake-slice/src/executor.rs
  line: 353
  category: contract-drift
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: files were written before git add/commit and never rolled back
- file: crates/intake-slice/src/executor.rs
  line: 325
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: a commit message containing NUL passed the check and failed after files were written
- file: crates/intake-slice/src/executor.rs
  line: 393
  category: judgement
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: the test command had no timeout and its output was buffered whole
- file: crates/intake-slice/src/executor.rs
  line: 384
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: tests.run and commit hooks run model-edited code with the operator rights
- file: crates/intake-slice/src/executor.rs
  line: 143
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: one TestRun could be submitted as evidence more than once
- file: crates/intake-slice/src/selector.rs
  line: 93
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: the transcript had no entry-count bound and inspected contents can imitate transcript lines
- file: crates/intake-slice/src/executor.rs
  line: 310
  category: judgement
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: inspect read ignored files such as secrets and sent them to the provider
- file: crates/intake-slice/tests/selector_executor.rs
  line: 129
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: the executor git in the acceptance test reads the operator global git config
```

| # | finding | decision |
|---|---|---|
| 1 | executor.rs:377 a failed edit leaves an ignored file; tree counts as clean; verifier submits pass about a HEAD without it (blocker) | accept, fix: the executor never writes or reads a path git ignores (`git check-ignore --no-index` style check per path, before anything is written or read); refused like an outside path, with its own `ExecuteError` variant |
| 2 | executor.rs:353 files written before add/commit, never rolled back | accept, fix: an edit is atomic: on any failure after the first write, restore every written path to its previous bytes (or remove it if it was new) and unstage it, so the tree is as before; the module doc says so |
| 3 | executor.rs:325 NUL in the commit message passes the check | accept, fix: refuse a message containing NUL (and other control characters except newline and tab) before anything is written |
| 4 | executor.rs:393 no timeout; whole output buffered | accept, fix: `TestCommand` carries a timeout (default 300 s, settable); the child is killed at the deadline and the run is `fail` with a timed-out marker; stdout and stderr are read incrementally and capped (keep the last 8 KiB each) |
| 5 | executor.rs:384 tests and hooks run model-edited code with the user's rights | decline (needs a sandbox, out of this slice); document in the module doc and in AGENTS.md's rules line as a known limit |
| 6 | executor.rs:143 one TestRun can be verified twice | accept, fix: the verifier refuses a second submission citing the same observation (a set of cited observation ids), returning a typed error |
| 7 | selector.rs:93 no bound on transcript entries; inspected contents can forge transcript lines | accept part: cap the transcript at the last 64 entries; forgery is a design note in the selector doc (evidence is unaffected) |
| 8 | executor.rs:310 inspect reads ignored files (secrets) and sends them to the provider | accept, fix: covered by row 1 (inspect refuses ignored paths) |
| 9 | tests/selector_executor.rs:129 executor's git reads the operator's global config in tests | decline: fix the test's doc comment only, as in case-frontier |

After the fixes: unit gate 73 tests; integration gate on 7e20e76: 99 passed, 2 ignored.
