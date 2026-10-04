---
format: aep.planning-md/3
id: review-result:parallel-safety-round-1
kind: review-result
status: active
title: Parallel-safety critic, round 1
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

- story:intent-references — the body has the slice hand references to Loom's selection context, and story:selector-executor (unordered with it, same wave 2) also builds the selector that is "given … its references". That is intake-slice work the scope omits, and neither body names the other. Surface inferred. Remedies: an ordering edge from one story to the other, recording the intake-slice selection-context code as the reason, or moving the hand-off into one story (selector-executor or slice-loop-cli) so intent-references stays inside `crates/intake-router` — `.engineering/planning/story/intent-references.md:30`
- The collisions the wave report lists are ordered by edges: case-frontier before selector-executor, case-frontier and selector-executor before slice-loop-cli, and router-classifier before intent-references. The wave 1 stories and model-access share no file.

What I read: 7 artifacts (the epic and six stories) through `aep plan artifact show` and `aep plan artifact waves --status draft`, plus the crate manifests, `lib.rs` files and root `Cargo.toml` in the tree. The root `Cargo.toml` uses the `crates/*` glob, so the new crates need no edit there. I placed all six stories on cited surfaces, with none inferred and none unplaced. The one collision finding rests on an inferred surface, because no body names the file for the hand-off.

What I could not establish, and what is outside my lane:
- **Which file the references hand-off lands in.** I could not tell whether it is `selector.rs`, `run.rs` or elsewhere in `intake-slice`.
- **Where the `ExtractedReference` type comes from.** I found no generated Rust type in the tree, so I could not tell whether selector-executor needs `intake-router` (a dependency on intent-references) to consume it. This is arguably a dependency question for the design critic.
- **`classify` returning references beside the pick.** This changes `classify.rs`, which router-classifier owns and intent-references' scope omits. router-classifier is ordered first, so it is not a collision, but the scope is incomplete. This is a scope question, outside my lane.
- **Edge reasons.** The `depends_on` edges carry no stated reason, so the shared files are not recorded as the cause of any ordering.

```findings
- file: .engineering/planning/story/intent-references.md
  line: 30
  category: parallel-safety
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: "the body has the slice hand references to Loom's selection context, which is intake-slice work that selector-executor (same wave 2, unordered) also claims and that is absent from this story's scope; the surface is inferred (no body names the file), and the fix is an ordering edge naming that reason or moving the hand-off into one story"
```
