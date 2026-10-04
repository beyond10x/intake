---
format: aep.planning-md/3
id: story:router-classifier
kind: story
status: implemented
title: Propose the ELS protocol an intent should run under
relations:
- decomposes: epic:intake-slice
- serves: vision:O2
- serves: vision:governed-autonomy
- depends_on: story:model-access
scope:
- confidence: cited
  path: crates/intake-router/Cargo.toml
- confidence: cited
  path: crates/intake-router/src/classify.rs
- confidence: cited
  path: crates/intake-router/src/lib.rs
- confidence: cited
  path: crates/intake-router/tests/classify.rs
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T17:41:04Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"review_outcome":1}}}
- {from: "proposed", to: "active", at: "2026-10-04T17:41:04Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":1}}}
- {from: "active", to: "implemented", at: "2026-10-04T18:01:54Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"test_result":1,"review_outcome":3,"verification":1}}}
---
## Outcome

`intake-router::classify(intent, model: &dyn llm_core::Model, threshold) -> Result<ProtocolPick,
RouterError>` proposes one protocol of the ELS registry for an intent. It lists
`els::registry::list()` with each protocol's Canon `description` and artifact descriptions, and
forces (through `intake_model::call_tool`) a `pick_protocol` tool whose `protocol` field is an enum
of the registry entries (`name@major`, e.g. `software-change@1`). It returns the pick with its
`confidence` and `reasons`. A pick outside the list, or a confidence below the threshold, is refused
(`RouterError::OutsideRegistry`, `RouterError::Unsure`) rather than guessed.

## Acceptance

`intents_pick_their_protocol`: against a recorded-response fake `llm_core::Model`, the intent "make
the failing test pass" picks `software-change@1`, "the checkout service is down" picks
`incident-response@1`, a recorded pick `deploy-everything@1` is refused as `OutsideRegistry`, and a
recorded confidence of 0.2 against a 0.5 threshold is refused as `Unsure`.

## ESS first

`intake.routing.ProtocolPick` is declared (`ess/domains/routing.yaml`). The first commit is the
named test, red because `classify` does not exist.

## Not in scope

Profiles and subjects (Atlas `epic:ga-intent-router` names them; no ELS protocol declares case
inputs yet). Generating a protocol when none fits. References: `classify` returns the pick only; the
slice composes it with the references (story:slice-loop-cli).
