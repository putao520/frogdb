# Security patch: lz4_flex RUSTSEC-2026-0041

- **Upstream**: swapvec 0.3.0 (crates.io) pins `lz4_flex = "0.10.0"`.
- **Advisory**: RUSTSEC-2026-0041 (high) — decompressing invalid data can leak
  uninitialized memory or reused output buffer.
- **Solution**: `lz4_flex >=0.11.6,<0.12.0` OR `>=0.12.1`.
- **This tree**: only dependency change is `lz4_flex` → `0.11.6`. Source code of
  swapvec is otherwise identical to 0.3.0.
- **Why not upgrade swapvec**: cozo 0.7.6 requires `swapvec = "0.3.0"` (^0.3);
  swapvec 0.4.x still pins lz4_flex 0.10.0 anyway.
- **Consumers**: cozo → gsc-frog-spec-graph (optional `cozo-engine` feature).
