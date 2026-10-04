# Intake

From an intent to a governed case. Intake reads what someone wants done — a prompt, a request, a
question — and proposes the protocol it should run under, picked from the protocols ELS ships. Its
first use is a vertical slice that then runs that case until it needs a human:

```console
b10x-intake run --workspace <git repository> "make the failing test pass"
```

1. A small model classifies the intent against the ELS registry (`software.change/1`,
   `incident.response/1`, …) and says why.
2. A case is opened on the picked protocol and Canon evaluates it.
3. Loom chooses the next admissible action with a model and writes its arguments.
4. A local executor performs it in the workspace and records the evidence (for example a test
   result bound to the new implementation revision).
5. Canon evaluates again, and the loop continues until the only useful action needs approval (a
   merge needs authority), nothing is admissible, or the step budget ends.

**Status: nothing is built yet.** The crates are empty; the plan is in the AEP store under
`.engineering/`. The slice executes `software.change/1` only, and only inside the workspace it is
given; it never merges or deploys.

## Build

```console
task check
```

Requires Rust 1.98 or newer and [go-task](https://taskfile.dev). Model calls need
`ANTHROPIC_API_KEY`; the tests make none.

## Licence

Apache-2.0.
