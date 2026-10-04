---
format: aep.planning-md/3
id: review-result:acceptance-round-2
kind: review-result
status: active
title: Acceptance critic, round 2
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

case-frontier — the fresh-case check says `repository.merge` is "blocked by `tests.pass`", but the merge precondition is the claim `implementation.verified`, so Canon's blocked reason names `implementation.verified` and the check cannot match — .engineering/planning/story/case-frontier.md:45 (els/protocols/software-change/1.yaml `repository.merge.precondition`; canon/crates/canon/src/eval/actions.rs:165-182, which names the precondition's own claim)

case-frontier — the outcome promises that `completion` reports only an outcome Canon holds legitimate, and that a new `implementation` commit bumps the case revision, but the acceptance observes neither, so both can ship wrong and the test passes — .engineering/planning/story/case-frontier.md:32 and :37 (the acceptance, lines 41-49, covers only the fresh case and the fixture frontiers)

case-frontier — the acceptance feeds "every state" of `chg-1842` through `EvidencePort`, but the `merge-approved` state adds only an authority grant, and `EvidencePort` carries none while the epic forbids the slice from supplying authority, so that state cannot be driven or observed as written — .engineering/planning/story/case-frontier.md:46 (els/fixtures/software-change/chg-1842.fixture.yaml, state `merge-approved`: `add_authority`, `add_evidence: []`)

intent-references — the outcome promises Jira keys found inside URLs, GitHub issue and pull-request URLs, GitLab issues and "any other URL", but the acceptance yields none of them, and the Jira URL is deduplicated into the bare key, so it passes even if URL extraction is absent — .engineering/planning/story/intent-references.md:26 (acceptance at :38)

What I read: 8 artifacts (epic:intake-slice, the six stories, review-result:acceptance-round-1), each in full with `aep plan artifact show <id>`. I also read `ess/domains/routing.yaml`, the intake `AGENTS.md`, `els/protocols/software-change/1.yaml`, the `chg-1842` fixture, Canon `eval/actions.rs` and Commission `ports/governor.rs` and `ports/evidence.rs`. All six round-1 findings are fixed: the stop-reason table, the `picked software-change@1` string, the inspect step, the transport and wrong-tool errors, the references handoff, and the derived revisions.

What I could not establish:
- Whether llm will ship `ToolChoice::Named` and a Codex credential: llm `openai-access` is not in the tree, which is a dependency question and out of my lane.
- Out of my lane (design): the story:model-access title still says "a Claude model" while its outcome is the Codex/gpt route. The protocol is named `software-change@1` in the router but `software.change/1` in the executor and `NoLocalExecutor`, and the mapping between the two is not stated.
- story:slice-loop-cli observes only `Refused (unsure)`, not the `OutsideRegistry` form, which is covered in story:router-classifier. I judged this adequate.

```findings
- file: .engineering/planning/story/case-frontier.md
  line: 45
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the fresh-case check says `repository.merge` is \"blocked by `tests.pass`\", but the merge precondition is the claim `implementation.verified`, so Canon's blocked reason names `implementation.verified` and the check cannot match"
- file: .engineering/planning/story/case-frontier.md
  line: 37
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the outcome promises that `completion` reports only an outcome Canon holds legitimate and that a new `implementation` commit bumps the case revision, but the acceptance observes neither, so both can ship wrong and the test passes"
- file: .engineering/planning/story/case-frontier.md
  line: 46
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the acceptance feeds \"every state\" of `chg-1842` through `EvidencePort`, but the `merge-approved` state adds only an authority grant, which `EvidencePort` does not carry and the slice must never supply, so that state cannot be driven or observed as written"
- file: .engineering/planning/story/intent-references.md
  line: 26
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: "the outcome promises Jira keys inside URLs, GitHub issue and pull-request URLs, GitLab issues and any other URL, but the acceptance yields none of them and the Jira URL is deduplicated into the bare key, so it passes even if URL extraction is absent"
```
