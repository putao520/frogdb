# frogdb

Fork of [cozodb/cozo](https://github.com/cozodb/cozo) maintained by the
frog-build project (`github.com/putao520/frogdb`).

## Provenance

- Forked from cozodb/cozo at commit `481af05`
  ("Merge pull request #286 from preludeorg/fix-stored-prefix-join").
- Upstream has been dormant since 2024-12; this fork is the maintenance line
  for Frog's embedded graph engine.
- Publisher crate: `cozo-core/` (package name `cozo`). Maintenance versions
  carry a `-frog.N` pre-release suffix (e.g. `0.7.6-frog.1`) and are consumed
  via path dependency by grok-build; they are not published to crates.io.

## Maintenance policy

Authorized by user ruling 2026-08-28:

- frog-build maintains this fork. **Bug fixes AND capability evolution are both
  authorized** — the target direction is a graph + document + vector hybrid
  database on the CozoScript base.
- For pure upstream bugs, issues are still filed against cozodb/cozo as a
  courtesy, with fixes carried here first (upstream is dormant and may not
  respond).
- No compatibility contract with upstream release cadence: divergence from
  upstream is expected and intentional from `frog` branch onward.

## Security notes

- **RUSTSEC-2026-0041** (lz4_flex < 0.11.6 leaks uninit / reused buffer on
  invalid input): `swapvec` 0.3.0 is vendored in-tree at
  `third_party/swapvec-0.3-lz4-fix` with `lz4_flex` bumped `0.10.0 → 0.11.6`
  (see `third_party/swapvec-0.3-lz4-fix/SECURITY-PATCH.md`).
  `cozo-core` consumes it via path dependency; upstream swapvec 0.3/0.4 still
  pin the vulnerable lz4_flex with no crates.io release past it.
