---
format: aep.planning-md/3
id: story:model-access
kind: story
status: implemented
title: Make one forced tool call through the llm crates over the Codex subscription
relations:
- decomposes: epic:intake-slice
- serves: vision:O2
- serves: vision:governed-autonomy
scope:
- confidence: cited
  path: crates/intake-model/Cargo.toml
- confidence: cited
  path: crates/intake-model/src/lib.rs
- confidence: cited
  path: crates/intake-model/tests/forced_tool_call.rs
revision: 10
transitions:
- {from: "draft", to: "proposed", at: "2026-10-04T15:17:43Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"review_outcome":2}}}
- {from: "proposed", to: "active", at: "2026-10-04T15:17:43Z", actor: "human:timo", revision: 9, decided_on: {"recorded":{"review_outcome":2}}}
- {from: "active", to: "implemented", at: "2026-10-04T17:42:18Z", actor: "human:timo", revision: 10, decided_on: {"recorded":{"test_result":1,"review_outcome":4,"verification":1}}}
---

## Outcome

`intake-model` gives Intake one model handle over the operator's Codex subscription and one helper
to force a typed answer from it:

- `codex_model(model_id) -> Result<impl llm_core::Model, ModelError>` builds the llm
  `ResponsesClient` for `https://chatgpt.com/backend-api/codex` with the llm `CodexAuthFile`
  credential (feature `codex-auth-file`). llm does no ambient lookup, so Intake expands the path:
  `$CODEX_HOME/auth.json`, else `$HOME/.codex/auth.json`, refused as `MissingCredential` when
  neither variable is set. The credential is read-only: Intake never writes the file and never
  refreshes the token. An expired token is `ExpiredCredential`, whose message says to run `codex`.
- `call_tool(model: &dyn llm_core::Model, instructions, items, tool) -> Result<serde_json::Value,
  ModelError>` forces one named tool (`ToolChoice::Named`) and returns that call's arguments.

Every other crate takes a `&dyn llm_core::Model`, so tests pass a recorded fake and only the CLI
calls `codex_model`.

## Acceptance

`a_forced_tool_call_returns_its_arguments`, against a local HTTP fixture of a Responses stream and a
fixture credential file: one `function_call` returns its arguments; a stream with no tool call gives
`NoToolCall`; a call to another tool gives `WrongTool`; a connection the fixture closes gives
`Transport`; a missing credential file gives `MissingCredential` and an expired fixture token gives
`ExpiredCredential`, both before any request.

## ESS first

None: no `intake.routing` noun changes. The first commit is the named test, red because
`intake-model` is empty.

## Live check

Not in CI. On the operator's machine, one `call_tool` against the Codex backend with a trivial
tool; the transcript records what the backend accepted or refused. A refusal that needs an llm
change (a request field or header the backend requires) is filed against llm before
`slice-loop-cli`'s live qualification.

## Depends on

llm `responses-client` and `codex-auth-file`, released as llm `0.1.5`; Intake pins that tag.
