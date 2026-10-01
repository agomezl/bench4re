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
- `proto/` Rust (prost + tonic) bindings for REv2.
- `toolchains/` the prost/tonic toolchain.
- `client/` the benchmark client.
- `tools/` lint (clippy aspect) and format targets.

## Common commands

```sh
bazel build --config=lint //...   # clippy
bazel run //:format               # rustfmt
bazel run //:bazelrc.update       # regenerate .global.bazelrc presets
bazel run @rules_rust//tools/rust_analyzer:gen_rust_project  # IDE support
```

Bazel is the only supported build: the REv2 bindings are split into one crate
per proto package (REv2, google/rpc, google/longrunning, ...), so they can't be
checked in as a single file for Cargo the way burst does with `analysis_v2`.

## Updating dependencies

- Rust crates: edit the `crate.spec` entries in `MODULE.bazel`, then
  `CARGO_BAZEL_REPIN=1 bazel mod tidy`.
- REv2 protos: bump `REMOTE_APIS_COMMIT` and its `sha256` in `MODULE.bazel`.
