---
format: aep.planning-md/3
id: review-result:adversary-w17-intake-slice-loop-cli-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w17 adversary, intake story:slice-loop-cli, pass 1
relations:
- reviews: story:slice-loop-cli
revision: 1
---
## Adversary pass (single) — intake story:slice-loop-cli

Tree: 8a77a8c + phase 2. Verdict NEEDS-CHANGE; cases 101→115, red 7. New file: `crates/intake-cli/tests/adversary_slice_run.rs` (14 cases).

```findings
- file: crates/intake-slice/src/run.rs
  line: 258
  category: acceptance
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: a selection Loom refuses was reported as NothingAdmissible while the frontier listed admissible actions
- file: crates/intake-slice/src/run.rs
  line: 348
  category: acceptance
  severity: blocker
  verdict: NEEDS-CHANGE
  origin: introduced
  message: repository.inspect of a missing file ended the whole run with an error
- file: crates/intake-slice/src/run.rs
  line: 344
  category: judgement
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: a refused action was never recorded in the briefing, so the model repeated it
- file: crates/intake-slice/src/run.rs
  line: 208
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: the refused line printed the classifier protocol raw, so escapes could forge a stop line
- file: crates/intake-slice/src/run.rs
  line: 218
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: pick reasons kept carriage returns and escape sequences
- file: crates/intake-slice/src/run.rs
  line: 418
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: effect lines printed file contents and test output with raw control characters
- file: crates/intake-cli/src/main.rs
  line: 65
  category: judgement
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: every stop reason exited 0
```

| # | finding | decision |
|---|---|---|
| 1 | run.rs:258 a selection Loom refuses is reported as NothingAdmissible | accept, fix: NothingAdmissible only when the frontier lists no admissible action; a selection Loom refuses (blocked or unlisted) is a refused step, recorded in the briefing (row 3), and the loop goes on within the step budget |
| 2 | run.rs:348 inspect of a missing file ends the run | accept, fix: a failed inspect read is a refused step with its reason, recorded in the briefing |
| 3 | run.rs:344 a refused action never reaches the briefing | accept, fix: add `Briefing::record_refusal(action, reason)` in crates/intake-slice/src/selector.rs (scope widened by the coordinator) and call it for every refused step; the next selection sees "refused: <reason>" |
| 4-6 | run.rs:208, :218, :418 model text, pick reasons, file contents and test output reach the terminal with raw control characters | accept, fix: one function escapes every C0/C1 control character except tab, and ESC/DEL, as visible escapes; every text from a model, a file or a command goes through it before printing |
| 7 | main.rs:65 every stop reason exits 0 | accept, fix: exit 0 for ApprovalRequired (the slice reached its human gate), 3 for NothingAdmissible, StepBudget, NoLocalExecutor and Refused, 1 for errors, 2 for usage; `--help` states it |
| 8 | fix pass: adversary case `an_unlisted_selection_is_not_reported_as_nothing_admissible` had max_steps 10 with one recorded reply; under row 1 the loop asks again | accept: the coordinator set that case to max_steps 1 (scratch/adversary-unlisted-budget.patch); its assertions are unchanged |
