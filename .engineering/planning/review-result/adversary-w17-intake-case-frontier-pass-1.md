---
format: aep.planning-md/3
id: review-result:adversary-w17-intake-case-frontier-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w17 adversary, intake story:case-frontier, pass 1
relations:
- reviews: story:case-frontier
revision: 1
---
## Adversary pass 1 — intake story:case-frontier

Tree: 7673c62 + phase 2. Verdict CONFIRMED; cases 48→56, red 3. New file: `crates/intake-slice/tests/adversary_case.rs`.

```findings
- file: crates/intake-slice/src/case.rs
  line: 136
  category: boundary
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: head() inherits GIT_DIR/GIT_WORK_TREE, so a slice started under a git hook records another repository HEAD
- file: crates/intake-slice/src/case.rs
  line: 137
  category: boundary
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: git discovery walks up, so a directory inside any repository opens a case on that repository HEAD, and a bare repository is accepted
- file: crates/intake-slice/tests/slice_case.rs
  line: 7
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: the test claims it reads nothing from the home directory, but the subject git reads the global config
```

| # | finding | decision |
|---|---|---|
| 1 | case.rs:136 head() inherits GIT_DIR / GIT_WORK_TREE | accept, fix: remove GIT_DIR, GIT_WORK_TREE, GIT_INDEX_FILE, GIT_COMMON_DIR, GIT_OBJECT_DIRECTORY and GIT_ALTERNATE_OBJECT_DIRECTORIES from the child environment of every git call the slice makes |
| 2 | case.rs:137 discovery walks up; bare repo accepted | accept, fix: the workspace is the root of a non-bare work tree: refuse when `rev-parse --is-bare-repository` is true or `--show-toplevel` (canonicalised) differs from the canonicalised workspace path, as `CaseError::Workspace`; doc comment says so |
| 3 | slice_case.rs:7 doc claims nothing is read from home, but case::open's git reads the global config | accept, fix the doc comment only: the test's own git calls ignore global/system config; the subject's git runs with the operator's normal git configuration (as the slice does in use). No in-process env mutation |
