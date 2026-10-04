---
format: aep.planning-md/3
id: story:intent-references
kind: story
status: draft
title: Extract an intent's key references so the next node starts with context
relations:
- decomposes: epic:intake-slice
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: crates/intake-references/Cargo.toml
- confidence: cited
  path: crates/intake-references/src/lib.rs
- confidence: cited
  path: crates/intake-references/tests/references.rs
revision: 5
---
## Outcome

The first-level node extracts the key elements an intent carries, so the next node starts with
context instead of rediscovering it. A crate of its own, `intake-references`, provides
`references(intent) -> Vec<ExtractedReference>`. It finds:

- Jira issue keys, both bare (such as `DEV-630`) and inside Jira URLs;
- Slack message permalinks;
- GitLab issue and merge-request URLs;
- GitHub issue and pull-request URLs;
- any other URL.

Each comes with its canonical value. Extraction is deterministic (patterns, no model call),
deduplicated and in text order. Handing the references on is the slice's work: the selector sees
them (story:selector-executor) and the transcript lists them (story:slice-loop-cli).

## Acceptance

`intent_references_are_extracted_in_order`: the intent "fix DEV-630, see
https://acme.atlassian.net/browse/DEV-630 and
https://gitlab.example.com/team/app/-/merge_requests/42 and the thread
https://example.slack.com/archives/C01/p1700000000000100, also https://acme.atlassian.net/browse/OPS-7,
https://gitlab.example.com/team/app/-/issues/9, https://github.com/beyond10x/intake/pull/3 and
https://example.org/design.pdf" yields seven references, in this order and once each:

- `JiraIssue DEV-630`
- `GitlabMergeRequest team/app!42`
- `SlackMessage C01/1700000000.000100`
- `JiraIssue OPS-7` (found only inside its URL)
- `GitlabIssue team/app#9`
- `GithubPullRequest beyond10x/intake#3`
- `Url https://example.org/design.pdf`

A lower-case `dev-630` and an e-mail address yield nothing.

## ESS first

`intake.routing.ExtractedReference` and `ReferenceKind` are declared (`ess/domains/routing.yaml`).
The first commit is the named test, red because `references` does not exist.

## Not in scope

Fetching what a reference points at (the issue text, the thread, the diff). That is preloading
through connectors, a later story once a connector for each source exists. Which further kinds ship
is the `UNMAPPED:` marker on `ReferenceKind`.
