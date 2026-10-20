# Vendored GPUI Component

Snapshot of `gpui-component 0.7.3` from crates.io, used only so Cadence can
round the client-side window frame. The root `Cargo.toml` redirects the
`gpui-component` package here through `[patch.crates-io]`.

## Local patch

`src/window_border.rs`:

- `BORDER_RADIUS` is `12px` instead of `0`;
- the frame, backdrop and content border round the bottom corners as well as
  the top ones.

Cadence already paints its own 12px content corners, so the rectangular child
clip that upstream mentions does not reintroduce square corners.

## Updating

Keep the version aligned with `gpui-base` in the root `Cargo.toml`. Copy the
crate source again and reapply the patch above:

```bash
rsync -a --exclude Cargo.lock \
  ~/.cargo/registry/src/index.crates.io-*/gpui-component-<version>/ \
  vendor/gpui-component/
```

## License

Apache-2.0; `LICENSE-APACHE` is kept from the published crate.
