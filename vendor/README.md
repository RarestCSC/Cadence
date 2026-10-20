# Vendored GPUI

This directory contains a snapshot of the standalone GPUI workspace published
by [`amtoaer/zed`](https://github.com/amtoaer/zed) at commit
`a417dfbfbb89b5a90e69b22ee593456301c37fe9`.

It is not the Zed editor: it contains only the 22 crates needed by
`gpui-pre 0.3.9` and `gpui-component 0.7.3`, with the patches described in
[`gpui/LYRUNE-GPUI.md`](gpui/LYRUNE-GPUI.md). The root `Cargo.toml` redirects
every `gpui-pre-*` package to these local paths through `[patch.crates-io]`.

## Why it is vendored

A Cargo git dependency fetches the whole repository, and this snapshot lives in
a Zed monorepo branch; that checkout is about 400 MB. The GPUI sources
themselves are about 13 MB. Vendoring keeps builds offline and avoids carrying
the editor's history. The root workspace excludes this directory and patches
all `gpui-pre-*` packages to its crate paths.

After a successful build, offline verification should work:

```bash
cargo check --offline --locked -p cadence --all-targets
```

## License

The GPUI crates are Apache-2.0. The snapshot keeps Zed's `LICENSE-APACHE`,
`LICENSE-GPL` and `LYRUNE-GPUI.md` for provenance.

## Updating

Fetch only the revision and copy the working tree:

```bash
git init /tmp/zed-gpui
git -C /tmp/zed-gpui remote add origin https://github.com/amtoaer/zed
git -C /tmp/zed-gpui fetch --depth 1 origin a417dfbfbb89b5a90e69b22ee593456301c37fe9
git -C /tmp/zed-gpui checkout --detach FETCH_HEAD
rsync -a --delete --exclude .git --exclude target --exclude Cargo.lock \
  /tmp/zed-gpui/ vendor/gpui/
```

After updating, check whether the workspace members changed and adjust the
paths under `[patch.crates-io]` in the root `Cargo.toml` if necessary.

`gpui-component` itself is a separate vendored snapshot under
[`gpui-component`](gpui-component); see its `README-CADENCE.md` for the local
window-border patch.
