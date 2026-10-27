# Vendored crates

## wgpu-hal 30.0.1 (`vendor/wgpu-hal`)

A copy of the crates.io `wgpu-hal` 30.0.1 source (MIT OR Apache-2.0, see its `LICENSE.*` files),
patched through `[patch.crates-io]` in the workspace `Cargo.toml`.

**Change:** `src/metal/adapter.rs`, the `msl_version` choice. On macOS 15.0+ wgpu-hal asks for Metal
language version 3.2. The Metal runtime compiler on macOS 15.x (for example 15.7.5) rejects
`-std=metal3.2` with `invalid value 'metal3.2'`, so every shader module fails, the wgpu device is
lost, and GridCraft quits with `Parent device is lost` before its first frame. The patch raises the
macOS threshold for 3.2 from 15.0 to 26.0, so macOS 15 uses 3.1, which that compiler accepts. The
iOS, tvOS and visionOS thresholds are unchanged.

**Why not upgrade:** 30.0.1 is the newest wgpu-hal on crates.io.

**Remove this copy** when a released wgpu-hal picks a Metal language version the running OS
accepts. Check with `cargo update -p wgpu-hal` and a run on macOS 15.

**Scope:** the copy is excluded from the workspace (`exclude` in `Cargo.toml`), so workspace lints
and `cargo xtask` checks don't apply to it. Keep the diff against upstream to the one change above.
