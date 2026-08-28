# frogdb

Fork of [cozodb/cozo](https://github.com/cozodb/cozo) maintained by the
frog-build project (`github.com/putao520/frogdb`).

## Provenance

- Forked from cozodb/cozo at commit `481af05`
  ("Merge pull request #286 from preludeorg/fix-stored-prefix-join").
- Upstream has been dormant since 2024-12; this fork is the maintenance line
  for Frog's embedded graph engine.
- Publisher crate: `cozo-core/` (package name `frogdb`). Maintenance versions
  carry a `-frog.N` pre-release suffix (for example, `0.7.6-frog.1`) and are
  prepared for publication to crates.io from this repository.

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
  invalid input): the `swapvec` 0.3.0 source is embedded in
  `cozo-core/src/swapvec/` and uses direct `lz4_flex` 0.11.6-or-newer
  dependency resolution. Upstream swapvec 0.3/0.4 still pins the vulnerable
  lz4_flex release with no crates.io release past it.
