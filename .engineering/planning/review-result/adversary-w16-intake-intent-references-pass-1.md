---
format: aep.planning-md/3
id: review-result:adversary-w16-intake-intent-references-pass-1
kind: review-result
status: active
title: Wave 2026-10-04-w16 adversary, intake story:intent-references, pass 1
relations:
- reviews: story:intent-references
revision: 1
---
I'm returning NEEDS-CHANGE on intake/intent-references: 12 of my 26 new cases fail against the extractor. The full gate and the mutant runs were not done, because free disk fell from 12G to 6.1G partway through (other sessions on the machine; this unit's build dir totals 49M) and the brief says to stop builds under 10G.

```
unit: intake/intent-references, working tree on d4e18ea (phase 2 uncommitted) + tests/adversary_references.rs
verdict: NEEDS-CHANGE
cases: executed 4→30 (4 = implementor's reported count; 26 = my file run alone; whole suite not re-run, disk), red 12
origin: introduced 13 / pre-existing 0 / undecided 0
wrote-outside-worktree: 2 paths
needs-coordinator: yes. The full gate (clippy, cargo test --workspace) and the mutant probes were not run: free disk is 6.1G, under the 10G stop line.
```

**1. Diff stat**

`git --no-pager diff --stat` lists only the implementor's phase-2 files: Cargo.lock, Cargo.toml, src/lib.rs, tests/references.rs and ess/domains/routing.yaml (5 files, +188 −6). My only change is a new untracked test file, `crates/intake-references/tests/adversary_references.rs`. I changed no file that isn't a test.

**2. Cases** — `~/.local/state/worktree/trees/b10x/intake/intake-w16-intent-references/crates/intake-references/tests/adversary_references.rs`, run alone: `test result: FAILED. 14 passed; 12 failed` (log: `scratch/adversary-p1/red-alone.log`).

| Red case (actual result, verbatim) | Cause at lib.rs |
|---|---|
| `a_cve_identifier_is_not_truncated_into_a_jira_key`: `[(JiraIssue, "CVE-2024")]` | :41 `\b` allows a `-digit` right after the key |
| `standard_names_are_not_jira_keys`: `[(JiraIssue, "UTF-8")]` (also ISO-8601, SHA-256, HTTP-2) | :41 |
| `a_url_with_balanced_parentheses_keeps_its_closing_parenthesis`: `"…/Rust_(programming_language"` | :91 strips `)` even when the parentheses are balanced |
| `a_markdown_link_whose_label_is_its_url_yields_the_reference_once`: `(Url, "https://github.com/…/pull/3](https://github.com/…/pull/3")` | :41 `]` and `(` are allowed inside a URL |
| `markdown_emphasis_around_a_url_is_not_part_of_it`: `(Url, "…/pull/3**")` | :91 |
| `a_slack_formatted_link_with_a_label_yields_the_reference`: `(Url, "…/pull/3\|PR")` | :41 `\|` is allowed inside a URL |
| `a_bare_scheme_is_not_a_url`: `[(Url, "https://")]` | :41 and :91 |
| `keys_separated_by_a_slash_are_both_found`: only `DEV-1` | :99 the `/` guard drops `DEV-2` |
| `a_key_followed_by_cjk_text_is_found`: `[]` for `DEV-630を…` | :41 Unicode `\b` sees no boundary before を |
| `an_upper_case_scheme_is_a_url`: `[]` for `HTTPS://…` | :41 matching is case-sensitive |
| `a_github_repository_dedups_across_owner_and_name_case`: two entries | :112 owner and repo are not lower-cased |
| `a_plain_url_dedups_across_host_case`: two entries | :126 the host is not lower-cased |

The 14 green cases cover every attack you listed that didn't break. Because of the disk stop, none of them has been shown to fail against a mutant.

**3. Gate**

| Command | Exit |
|---|---|
| `cargo fmt --all --check` | 0 (I ran `rustfmt` on my own file first) |
| `ess specify validate --path ess --strict-requires` | 0, `intake v1 — 2 file(s), valid` |
| `cargo clippy …` / `cargo test --workspace --locked` / `-- --list` | **not run**: disk was at 8.9G, then 6.1G |

**4. Judgement and reach**
- Nothing in the tree calls `references()` yet; the selector and the transcript are later stories. "What reaches it" for every row is therefore the public function only.
- `ess/domains/routing.yaml:10` is outside the story's scope. The edit also removed the `UNMAPPED:` marker, which the story names as the place where further kinds are tracked.
- The Slack `<url|label>` case is INFEASIBLE: I found no Slack text source.

**5. Not broken** (one line each)
- A key followed by `:`, and `AB2-12`.
- Key shapes inside words (`xPR-1`, `foo_PR-1`, `ÄPR-1`, `PR-1x`), and `ops@DEV-1`.
- `.` `,` `)` `>` `]` `"` `'` after a URL, and `[text](url)`.
- A query plus fragment on a plain URL; a Jira URL with a query.
- Slack `?thread_ts=`, GitLab `a/b/c/-/merge_requests/1/diffs`, GitHub `/files` and `#issuecomment`.
- Dedup of URL and bare key; http vs https; `GitHub.com` and `www.github.com`.
- Speed: 1 MiB of Unicode text takes 1.24s in debug and 108ms in release; a 1 MiB URL takes 1.61s and 134ms. It grows linearly.

**6. Written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w16/intake-intent-references/scratch/adversary-p1/` (holds `red-alone.log`). I made no mutant copies, so there were none to delete.
- `~/.cache/b10x-target/intake-w16-intent-references/release/`: my release timing run. The build directory is shared, so I left it in place.

**7. Findings**

```findings
- file: crates/intake-references/src/lib.rs
  line: 41
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "CVE-2024-1234 is extracted as JiraIssue CVE-2024, a truncated value that is neither a key nor the identifier in the text"
- file: crates/intake-references/src/lib.rs
  line: 41
  category: boundary
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "UTF-8, ISO-8601, SHA-256 and HTTP-2 are each extracted as a JiraIssue"
- file: crates/intake-references/src/lib.rs
  line: 91
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "trim_url strips a balanced closing parenthesis, so a Wikipedia-style URL loses its last character"
- file: crates/intake-references/src/lib.rs
  line: 41
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "a markdown link whose label is its URL becomes one Url containing ']('; the GitHub pull request is lost"
- file: crates/intake-references/src/lib.rs
  line: 91
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "markdown emphasis around a URL stays on it, so a GitHub pull request is classified as Url ending in '**'"
- file: crates/intake-references/src/lib.rs
  line: 41
  category: boundary
  severity: note
  verdict: INFEASIBLE
  origin: introduced
  message: "a Slack-formatted <url|label> link becomes Url '...|PR'; no Slack text source exists in the tree"
- file: crates/intake-references/src/lib.rs
  line: 41
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "the text 'https://...' yields a Url with the bare value 'https://'"
- file: crates/intake-references/src/lib.rs
  line: 99
  category: boundary
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "the '/' guard drops the second key in 'DEV-1/DEV-2'"
- file: crates/intake-references/src/lib.rs
  line: 41
  category: boundary
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "a key directly followed by CJK text is not found, because the Unicode word boundary does not fire"
- file: crates/intake-references/src/lib.rs
  line: 41
  category: boundary
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "a URL with an upper-case scheme (HTTPS://) is not extracted at all"
- file: crates/intake-references/src/lib.rs
  line: 112
  category: property
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "GitHub owner and repository are not case-folded, so one pull request yields two references"
- file: crates/intake-references/src/lib.rs
  line: 126
  category: property
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "a plain Url keeps its host case, so one URL that differs only in host case yields two references"
- file: ess/domains/routing.yaml
  line: 10
  category: judgement
  severity: warning
  verdict: CONFIRMED
  origin: introduced
  message: "an edit outside the story's typed scope removes the UNMAPPED marker that the story names as where further kinds are tracked"
```
