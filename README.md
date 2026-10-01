# bench4re

A benchmark for remote execution backends speaking the
[REv2 protocol](https://github.com/bazelbuild/remote-apis) (the one Bazel uses):
a custom client sends benchmark workloads to an RE endpoint and measures it.

## Building

Built with Bazel (via [bazelisk](https://github.com/bazelbuild/bazelisk); the
version is pinned in `.bazelversion`).

```sh
bazel build //...
bazel run //client -- grpc://localhost:8980 [instance_name]
```

## Layout

- `third_party/remote_apis/` BUILD overlay for bazelbuild/remote-apis
  (fetched in `MODULE.bazel`, pinned to a commit).
- `proto/` Rust (prost + tonic) bindings for REv2 and the prost toolchain.
- `client/` the benchmark client.

## Updating dependencies

- Rust crates: edit the `crate.spec` entries in `MODULE.bazel`, then
  `CARGO_BAZEL_REPIN=1 bazel sync --only=crates`.
- REv2 protos: bump `REMOTE_APIS_COMMIT` and its `sha256` in `MODULE.bazel`.
- rust-analyzer: `bazel run @rules_rust//tools/rust_analyzer:gen_rust_project`.
