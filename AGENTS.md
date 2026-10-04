# AGENTS.md — intake

What Intake is and how to run it is in [README.md](README.md); this file is what an agent changing it
must know. The cross-repository architecture is Atlas ADRs 0066–0075 and Atlas
`epic:ga-intent-router`.

## Serves

- **O2 — decisions as data, with evidence.** Which protocol a piece of work runs under is a recorded
  proposal with its reasons, not an implicit choice.

## Boundary

- Intake owns intent routing: turning an intent into a proposed protocol (and later a profile and a
  subject). The proposal is never authority; whoever opens the case checks it.
- Protocols belong to ELS, their semantics to Canon, agent execution to Commission and Loom. Intake
  reads the ELS registry and calls Canon, Commission and Loom; it re-implements none of them.
- `crates/intake-slice` is a temporary vertical slice: its case opening, Canon-to-frontier mapping,
  loop and local executor exist only until Commission's local runtime loop and a governor replace
  them. Keep it small; do not grow it into a runtime.
- The slice executes `software.change/1` actions inside the given workspace only. It never merges,
  pushes or deploys, and never supplies authority on the operator's behalf.
- Evidence comes from a trusted verifier (the slice runs the test command itself), never from what a
  model says happened (Atlas ADR 0074). The case is governed by `beyond10x/governor` (Canon behind
  Commission's `Governor` and `EvidencePort`); Intake never evaluates Canon itself.

## Rules

- Anything that runs is Rust; command lines use clap derive.
- Model calls go through the `llm` crates (`b10x-llm-*` at a pinned tag), never a hand-written HTTP
  client. The credential is the operator's Codex subscription (`~/.codex/auth.json`, refreshed
  through `auth.openai.com`), read and renewed by llm's credential, never by Intake code; tests
  read no credential file.
- A model's choice is checked against the list it was given; a pick outside it is refused.
- Tests make no model or network call: they use recorded responses and local fixtures.
- No `/home/<name>/` path literals anywhere: common Gates personal-paths has no allowance.

## ESS

New nouns get their ESS domain before stories are written around them: `ess/` holds the
`intake.routing` domain. A specification change lands first (Atlas ADR 0080), with a test that fails
on it, then the implementation.

## Work

- Planned in the AEP store under `.engineering/`, written only through `aep plan artifact`. Body
  drafts go in `.engineering/drafts/` (ignored).
- Build with `CARGO_TARGET_DIR=$HOME/.cache/b10x-target/intake` (the Taskfile sets it); give each
  worktree its own `CARGO_TARGET_DIR` before trusting a gate run there.
- Every commit and push is `b10x-bot[bot]`'s through `b10x-gates bot`; every GitHub write goes
  through `b10x-gates api`.
- Use a managed worktree (`worktree create --repo intake --purpose …`) for changes.
