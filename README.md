# vertex-dev-generator-plane

`vertex-dev-generator-plane` is a compact Rust repository for developer tools, centered on this goal: Build a Rust toolkit that studies generator behavior through negative fixtures, with human-readable error snapshots and local-only command execution.

## Why I Keep It Small

The point is to make a small domain rule concrete enough that a reader can change it and immediately see what broke.

## Vertex Dev Generator Plane Review Notes

Start with `diagnostic quality` and `safe rewrite`. Those cases create the widest score spread in this repo, so they are the best quick check when the model changes.

## Included Behavior

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/vertex-dev-generator-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `diagnostic quality` and `safe rewrite`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Internal Model

The repository has two validation layers: the original compact policy fixture and the domain review fixture. They are separate so one can change without hiding failures in the other.

The Rust code keeps the review rule close to the tests.

## Try It Locally

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Validation

The verifier is intentionally local. It should fail if the fixture score math, lane assignment, or language-specific test drifts.

## Scope

The fixture set is small enough to audit by hand. The next useful expansion is malformed input coverage, not extra surface area.
