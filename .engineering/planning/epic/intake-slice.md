---
format: aep.planning-md/3
id: epic:intake-slice
kind: epic
status: implemented
title: 'Vertical slice: from an intent to a governed case run until blocked'
relations:
- serves: vision:O2
- serves: vision:governed-autonomy
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T18:39:20Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"review_outcome":3,"verification":1}}}
- {from: "proposed", to: "active", at: "2026-10-04T18:39:20Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"review_outcome":3,"verification":1}}}
- {from: "active", to: "implemented", at: "2026-10-04T18:39:20Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":3,"verification":1}}}
---
## Outcome

`b10x-intake run --workspace <git repository> "<intent>"` takes an intent, extracts its key
references, proposes the ELS protocol it should run under, opens a case on that protocol and runs
it — Canon evaluates, Loom chooses the next admissible action with a model, a local executor
performs it and records evidence — until the only useful action needs approval, nothing is
admissible, or the step budget ends. It prints every step and why it stopped.

## Why

No path from an intent to a governed case exists today: the only prompt launcher
(`b10x-harness run --input`) knows no protocol, no intent router exists (Atlas
`epic:ga-intent-router`, draft), Commission's runtime loop is empty and its only governor is a fake.
This slice proves the chain end to end before those components exist, and gives the intent router a
home.

## Domain

The nouns are declared in `ess/domains/routing.yaml` (`intake.routing`): `Intent`,
`ExtractedReference`, `ProtocolPick`, `SliceRun`; `ess specify validate --path ess --strict-requires`
passes. One `UNMAPPED:` marker: which reference kinds beyond the first set ship.

## Bounds

- Executes `software.change/1` only, only inside the given workspace; never merges, pushes or
  deploys, never supplies authority. Other protocols are classified and evaluated, then stop with
  `NoLocalExecutor`.
- `crates/intake-slice` is temporary: Commission's `story:local-runtime-loop` and a governor replace
  it. The router (`intake-router`) and the references extractor stay.
- Models through the `b10x-llm-*` crates over the operator's Codex subscription: the OpenAI
  Responses wire at `https://chatgpt.com/backend-api/codex`, the token from `~/.codex/auth.json`
  refreshed through `auth.openai.com`. llm 0.1.4 has neither a Responses client nor that credential;
  both land in llm first (llm `openai-access`), and intake pins the llm release that ships them.
  Every role (classifier, selector, argument generator) uses `gpt-5.6-sol`, the model Harness has
  run on this route; the classifier's model is configurable so a smaller one can replace it once
  one is verified on the route. Tests make no model call.

## Done when

The offline slice acceptance (story slice-loop-cli) passes in CI, and one live run on a scratch
repository is recorded as evidence on this epic: `picked software-change@1`, at least one
`repository.edit`, a `tests.run` with `test_result pass`, stop `ApprovalRequired`
(`repository.merge`).

## Shared lockfile

`Cargo.lock` is touched by every story that adds a dependency and is not in any story's typed scope:
at each merge into a wave branch it is regenerated (`cargo metadata --offline` after taking either
side), and the integration gate runs `--locked` on the result.
