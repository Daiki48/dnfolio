---
title: Setting up monorepo development in Rust
description: A warning occurred when developing monorepo in Rust.
tags:
  - Tech
  - Rust
  - monorepo
---

## Warning

```shell
warning: virtual workspace defaulting to `resolver = "1"` despite one or more workspace members being on edition 2021 which implies `resolver = "2"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
note: for more details see https://doc.rust-lang.org/cargo/reference/resolver.html#resolver-versions
```

> The resolver is a global setting for a workspace, and the setting is ignored in dependencies. The setting is only honored for the top-level package of the workspace. If you are using a virtual workspace, you will still need to explicitly set the resolver field in the [workspace] definition if you want to opt-in to the new resolver.

[Details | Default Cargo feature resolver](https://doc.rust-lang.org/edition-guide/rust-2021/default-cargo-resolver.html#details)

## Resolved

In the root `Cargo.toml`, `resolver = "2"` was written.

```toml
[workspace]
resolver = "2" # Add line
members = [
"auth",
]
```

I am a first time monorepo developer.
