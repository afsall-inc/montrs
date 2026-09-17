# MontRS Dev-Hotpatch

Rust hot-patching toolchain for the MontRS dev server.

Captures the workspace's `rustc`/link invocations, links the hot-patchable
("fat") binary and the patch shared library, builds jump tables, and provides
the dev hub, the native client, and the `montrs-rustc-wrapper` /
`montrs-link-wrapper` shims.

Dev-only; applications use [`montrs-hotpatch`](../hotpatch/README.md) instead.

See [docs/tooling/hot-reload.md](../../docs/tooling/hot-reload.md).
