---
format: aep.planning-md/3
id: story:model-access
kind: story
status: draft
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
revision: 6
---
## Outcome

`intake-model` gives Intake one model handle over the operator's Codex subscription and one helper
to force a typed answer from it:

- `codex_model(model_id) -> Result<impl llm_core::Model, ModelError>` builds the llm Responses
  client for `https://chatgpt.com/backend-api/codex` with the llm Codex subscription credential
  (`~/.codex/auth.json`, refreshed through `auth.openai.com`). Both come from llm (`openai-access`);
  this story pins the llm release that ships them.
- `call_tool(model: &dyn llm_core::Model, instructions, items, tool) -> Result<serde_json::Value,
  ModelError>` forces one named tool (`ToolChoice::Named`) and returns that call's arguments.

Every other crate takes a `&dyn llm_core::Model`, so tests pass a recorded fake and only the CLI
calls `codex_model`.

## Acceptance

`a_forced_tool_call_returns_its_arguments`, against a local HTTP fixture of a Responses stream and a
fixture credential file: one `function_call` returns its arguments; a stream with no tool call gives
`NoToolCall`; a call to another tool gives `WrongTool`; a connection the fixture closes gives
`Transport`; a missing credential file gives `MissingCredential` before any request.

## ESS first

None: no `intake.routing` noun changes. The first commit is the named test, red because
`intake-model` is empty.

## Depends on

llm `openai-access` (a Responses client and the Codex subscription credential) released at a tag.
