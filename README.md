# vertex-dev-generator-plane

`vertex-dev-generator-plane` treats developer tools as a local verification problem. The Rust implementation is intentionally narrow, but the fixtures and notes make the behavior explicit.

## Vertex Dev Generator Plane Checkpoints

Treat the compact fixture as the contract and the extended examples as a scratchpad. The code should stay boring enough that a change in behavior is obvious from the test output.

## What This Is For

I use this kind of project to make a rule visible before adding more machinery around it. The important part here is not the size of the codebase. It is that the input signals, scoring rule, fixture data, and expected output can all be checked in one sitting.

## Project Layout

- `src`: primary implementation
- `tests`: verification harness
- `fixtures`: compact golden scenarios
- `examples`: expanded scenario set
- `metadata`: project constants and verification metadata
- `docs`: operations and extension notes
- `scripts`: local verification and audit commands
- `Cargo.toml`: Rust package metadata

## Useful Pieces

- Includes extended examples for safe defaults, including `recovery` and `degraded`.
- Documents repeatable output tradeoffs in `docs/operations.md`.
- Runs locally with a single verification command and no external credentials.
- Stores project constants and verification metadata in `metadata/project.json`.
- Adds a repository audit script that checks structure before running the language verifier.

## Architecture Notes

The core is a scoring model over demand, capacity, latency, risk, and weight. That keeps code shape, diagnostics, and safe defaults in one explicit decision path. The threshold is 174, with risk penalty 4, latency penalty 2, and weight bonus 2. The Rust code keeps ownership and data movement plain, which makes the tests useful for checking both behavior and API shape.

## Tooling

The only required setup is the local Rust toolchain. After cloning, stay in the repo root so fixture paths resolve correctly.

## Case Study

`boundary` is the first example I would inspect because it lands on the `review` path with a score of 123. The broader file also keeps `degraded` at 8 and `recovery` at 240, which gives the model a useful low-to-high spread.

## Local Workflow

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

This runs the language-level build or test path against the compact fixture set.

## Quality Gate

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/audit.ps1
```

The audit command checks repository structure and README constraints before it delegates to the verifier.

## Expansion Ideas

- Split the scoring constants into a typed configuration object and validate it before use.
- Add a comparison mode that shows how decisions change when one signal is adjusted.
- Add a loader for `examples/extended_cases.csv` and promote selected cases into the language test suite.
- Add one more developer tools fixture that focuses on a malformed or borderline input.

## Scope

The examples cover useful edges, not every edge. A larger version would add malformed-input tests, richer reports, and deeper domain parsers.
