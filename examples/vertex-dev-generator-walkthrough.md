# Vertex Dev Generator Plane Walkthrough

The fixture is intentionally compact, so the review starts with the cases that pull farthest apart.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 175 | ship |
| stress | diagnostic quality | 227 | ship |
| edge | review cost | 215 | ship |
| recovery | safe rewrite | 174 | ship |
| stale | change width | 199 | ship |

Start with `stress` and `recovery`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

If `recovery` becomes less cautious without a clear reason, I would inspect the drag input first.
