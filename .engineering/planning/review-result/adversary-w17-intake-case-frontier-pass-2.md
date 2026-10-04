---
format: aep.planning-md/3
id: review-result:adversary-w17-intake-case-frontier-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w17 adversary, intake story:case-frontier, pass 2
relations:
- reviews: story:case-frontier
revision: 1
---
## Adversary pass 2 — intake story:case-frontier

Tree: pass-1 tree + fixes. Verdict CONFIRMED; cases 56→60, red 2. New file: `crates/intake-slice/tests/adversary2_case.rs`.

```findings
- file: crates/intake-slice/src/case.rs
  line: 208
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: the toplevel was trimmed, so a work-tree root whose name ends in whitespace was refused
- file: crates/intake-slice/src/case.rs
  line: 207
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: git() required UTF-8 stdout, so a work-tree root at a non-UTF-8 path was refused
- file: crates/intake-slice/src/case.rs
  line: 154
  category: mutant
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: the bare-repository check was unobservable; --show-toplevel already refuses a bare repository
```

| # | finding | decision |
|---|---|---|
| 1 | case.rs:208 trim drops trailing whitespace of --show-toplevel | accept, fix: read the toplevel as raw bytes and strip exactly one trailing `\n` |
| 2 | case.rs:207 git() requires UTF-8 stdout; non-UTF-8 root refused | accept, fix: the toplevel is an OsStr from raw bytes (`OsStr::from_bytes`, unix); only the HEAD object name must be UTF-8 |
| 3 | case.rs:154 bare check unobservable | accept, fix: delete the bare check; `--show-toplevel` refuses a bare repository; the doc says so |

After the fixes: integration gate on 286a0cb: fmt, clippy, 60 tests passed, ess validate --strict-requires, aep validate; all exit 0.
