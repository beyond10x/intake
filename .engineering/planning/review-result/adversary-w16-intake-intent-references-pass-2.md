---
format: aep.planning-md/3
id: review-result:adversary-w16-intake-intent-references-pass-2
kind: review-result
status: active
title: Wave 2026-10-04-w16 adversary, intake story:intent-references, pass 2
relations:
- reviews: story:intent-references
revision: 1
---
I'm returning NEEDS-CHANGE on intake/intent-references: 8 of my 15 new cases fail against the pass-1 fixes. None of them repeats a pass-1 finding. The things you asked about specifically hold: `RFCX-1` and `ISO2-5` are found as keys, the standard-name list is not applied to keys inside Jira URLs, nested and sentence parentheses work, and path case is kept.

```
unit: intake/intent-references, working tree d4e18ea + uncommitted phase 2 and pass-1 fixes + tests/adversary2_references.rs
verdict: NEEDS-CHANGE
cases: executed 32→47, red 8
origin: introduced 8 / pre-existing 0 / undecided 0
wrote-outside-worktree: 1 directory (~/.cache/ga-wave-2026-10-04-w16/intake-intent-references/scratch/adversary-p2/)
needs-coordinator: yes. Accept or decline rows 1–8. The gate's `cargo test` is fail-fast, so I counted with a second run using --no-fail-fast.
```

**1. Diff stat**

`git --no-pager diff --stat` is unchanged from what I was handed: 5 files, +274 −6, all the implementor's. My only file is the new untracked `crates/intake-references/tests/adversary2_references.rs`. I wrote no file that isn't a test.

**2. Cases.** I wrote the file first, then ran it alone: `test result: FAILED. 7 passed; 8 failed` (`scratch/adversary-p2/red-alone.log`).

| Red case | Actual result (verbatim) |
|---|---|
| `a_key_followed_by_a_version_decimal_is_not_a_key` | `"relicense under MPL-2.0 this week"` → `[(JiraIssue, "MPL-2")]` |
| `a_url_template_keeps_its_closing_brace` | `[(Url, "https://api.example.org/v1/items/{id")]` |
| `cjk_sentence_punctuation_after_a_url_is_dropped` | `Some((Url, "https://github.com/beyond10x/intake/pull/3。"))` |
| `slack_italic_underscores_around_a_key_do_not_glue_it` | `_DEV-630_` → `[]` |
| `slack_italic_underscores_around_a_url_are_not_part_of_it` | `[(Url, "https://github.com/beyond10x/intake/pull/3_")]` |
| `a_decomposed_letter_before_a_key_glues_it_like_the_composed_one` | `A\u{0308}PR-1` → `[(JiraIssue, "PR-1")]`, while the composed `ÄPR-1` gives `[]` |
| `a_key_bordered_by_lao_text_is_found` | `ແກ້DEV-630ດ່ວນ` → `[]` |
| `japanese_text_glued_after_a_url_is_not_part_of_it` | `[(Url, "https://github.com/beyond10x/intake/pull/3を見てください")]` |

Each green case was killed by its own mutant, in one scratch copy with its own target dir (`mutants.log`):

| Green case | Mutant that turned it red |
|---|---|
| `a_project_that_only_starts_with_a_standard_name_is_a_key` (RFCX-1, ISO2-5, …) | m1: the list matches by `starts_with` |
| `a_jira_url_is_authoritative_for_a_standard_like_project` | m2: the list is also applied inside Jira URLs |
| `a_key_glued_to_a_non_ascii_letter_or_digit_is_not_a_key` | m3: `joins_word` is ASCII-only |
| `keys_next_to_emoji_and_symbols_are_found` | m4: every non-space, non-ASCII-punctuation character joins |
| `nested_parentheses_in_a_url_are_kept`, `a_url_in_parentheses_inside_a_sentence_…` | m5: strip `)` only when the URL has no `(`; m6: `>=` instead of `>` |
| `path_case_is_kept_outside_github_owner_and_repository` | m7: lower-case the whole URL; m8: lower-case the GitLab path |

I deleted the copy and its target dir (21M) afterwards.

**3. Gate.** Run in the worktree after the cases existed (`scratch/adversary-p2/gate.log`). Free disk was 15G.

| Command | Exit |
|---|---|
| `cargo fmt --all --check` | 0 |
| `cargo clippy --workspace --all-targets --locked -- -D warnings` | 0 |
| `cargo test --workspace --locked` | 101; it stopped at `adversary2_references` with `FAILED. 7 passed; 8 failed` |
| `ess specify validate --path ess --strict-requires` | 0, `intake v1 — 2 file(s), valid` |

With `--no-fail-fast` (exit 101, `gate-nff.log`) the test summary lines are:
- `adversary2_references`: `FAILED. 7 passed; 8 failed`
- `adversary_references`: `ok. 26 passed; 0 failed`
- `references`: `ok. 6 passed; 0 failed`
- the other 10 lanes: `ok. 0 passed`

`-- --list` shows 15, 26 and 6 tests in those three lanes.

**4. Findings.** All rows cover the working tree above. Nothing outside `crates/intake-references` calls `references()` yet, so for every row the only thing that reaches it is the public function.

| # | lib.rs | Verdict | Finding | Fix (I did not apply it) |
|---|---|---|---|---|
| 1 | :150 | NEEDS-CHANGE | `MPL-2.0`, `GPL-3.0-only` and `LGPL-2.1` (licence names) become keys. The `-<digit>` rule doesn't cover a `.<digit>` tail. | also reject a key followed by `.<digit>` |
| 2 | :110 | NEEDS-CHANGE | `}` is always stripped, so `…/items/{id}` loses its last character. The docs at :15 call it sentence punctuation. | balance `}` like `)` |
| 3 | :110 | NEEDS-CHANGE | `。`, `、` and `）` after a URL are not dropped, so a GitHub pull request is classified as Url. | add the CJK and fullwidth closers to the strip list |
| 4 | :164 | NEEDS-CHANGE | `_` always joins a word, so Slack and Markdown italics `_DEV-630_` hide the key. The decisions file treats Slack as a named source. | `_` joins only when a word character is on its far side |
| 5 | :60 | CONFIRMED | `_url_` keeps the trailing `_`; the pass-1 fix covered `*` only. | strip a trailing `_` |
| 6 | :164 | CONFIRMED | A combining mark (Unicode category M) does not join, so `ÄPR-1` written decomposed (NFD) gives `PR-1`, while the composed form gives nothing. | treat category M as joining |
| 7 | :167 | CONFIRMED | Docs against behaviour: :11 and :161 say scripts written without spaces may border a key, but the list omits Lao, Khmer and Myanmar. | widen the list, or narrow the docs |
| 8 | :60 | CONFIRMED | Japanese text glued after a URL becomes part of it, so the pull request is lost. This is a trade-off against IRIs. | coordinator's call |

**5. Attacked and not broken**
- Prefix-sharing project keys (RFCX, ISO2, UTFS, SHAPE, MDM) are found.
- A Jira URL is authoritative for RFC-12 and UTF-8, including dedup with a bare `UTF-8`.
- Keys next to Cyrillic, Greek, Arabic-Indic and ASCII digits, and superscripts, are refused. Keys next to emoji, fullwidth parentheses, a zero-width space and CJK punctuation are found.
- Nested parentheses, a `)` after a balanced pair, and a URL inside parentheses in a sentence all come out right.
- Case folding is limited to scheme, host and GitHub owner/repository. GitLab and plain URL path, query and fragment case are kept.

**6. Written outside the worktree**
- `~/.cache/ga-wave-2026-10-04-w16/intake-intent-references/scratch/adversary-p2/` holds `red-alone.log`, `mutants.log`, `gate.log`, `gate-nff.log` and `sed/m1.sed` to `sed/m8.sed`. The copy and its target dir are deleted.
- The gate built into the unit's own assigned dir, `~/.cache/b10x-target/intake-w16-intent-references`.

```findings
- file: crates/intake-references/src/lib.rs
  line: 150
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "MPL-2.0, GPL-3.0-only and LGPL-2.1 yield JiraIssue keys because only a -<digit> tail is refused, not a .<digit> tail"
- file: crates/intake-references/src/lib.rs
  line: 110
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "a trailing } is stripped unconditionally, so a URL template ending in {id} loses its closing brace"
- file: crates/intake-references/src/lib.rs
  line: 110
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "CJK and fullwidth closing punctuation after a URL is not dropped, so a GitHub pull request is classified as a plain Url"
- file: crates/intake-references/src/lib.rs
  line: 164
  category: boundary
  severity: warning
  verdict: NEEDS-CHANGE
  origin: introduced
  message: "underscore always joins a word, so a Slack or Markdown italic _DEV-630_ yields no key"
- file: crates/intake-references/src/lib.rs
  line: 60
  category: boundary
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "an italic _url_ keeps the trailing underscore, so the GitHub pull request is classified as a Url"
- file: crates/intake-references/src/lib.rs
  line: 164
  category: boundary
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "a combining mark does not join, so decomposed (NFD) ÄPR-1 yields PR-1 while the composed form yields nothing"
- file: crates/intake-references/src/lib.rs
  line: 167
  category: contract-drift
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "the docs say scripts written without spaces may border a key, but Lao, Khmer and Myanmar are missing, so a key in Lao text is not found"
- file: crates/intake-references/src/lib.rs
  line: 60
  category: boundary
  severity: note
  verdict: CONFIRMED
  origin: introduced
  message: "Japanese text glued after a URL becomes part of it, so a GitHub pull request is classified as a plain Url"
```
