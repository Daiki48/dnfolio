---
title: Updated templates for Axum and Tera
description: I responded to an issue that was created in my past template projects for Axum and Tera.
tags:
  - Tech
  - Rust
  - Axum
  - Tera
---

## I have a project I created to try out with Axum and Tera

Over a year ago, I deployed a simple website with [Axum](https://docs.rs/axum/0.6.18/axum/) and [Tera](https://docs.rs/tera/1.19.0/tera/index.html).

[axum-tera | Daiki48](https://github.com/Daiki48/axum-tera)

The issue was created about a month ago. I was slow to notice.

[is unwrap() normal/necessary? #1](https://github.com/Daiki48/axum-tera/issues/1)

The question was about `unwrap()`.
On this occasion, I have updated this repository.

## Worked

- Update dependencies
- Added error handling
- Added test process

The corresponding PR is this.

[⚡ Added test and GitHub Actions, Added error handling, Update dep… #2](https://github.com/Daiki48/axum-tera/pull/2)

## Dependencies list

I updated `Cargo.toml`.

### Before

```toml
[dependencies]
axum = "0.6.18"
tokio = { version = "1.29.1", features = ['full']}
hyper = { version = "0.14.27", features = ['full']}
tower = { version = "0.4.13", features = ['full']}
axum-template = "1.0.0"
tera = "1.19.0"
```

### After

`dev-dependencies` for testing were also added.

```toml
[dependencies]
axum = { version = "0.7.9", features = ["macros"] }
tokio = { version = "1.42.0", features = ['full']}
hyper = { version = "1.5.2", features = ['full']}
tower = { version = "0.5.2", features = ['full']}
axum-template = "2.4.0"
tera = "1.20.0"
axum-macros = "0.4.2"

[dev-dependencies]
reqwest = { version = "0.12.11", features = ["json"] }
tokio = { version = "1.42.0", features = ["full"] }
```

## Added GitHub Actions

Added tests to be checked with GitHub Actions.

Added `.github/workflows/check.yaml` file.

```yaml
name: Rust

on:
  push:
    branches:
      - main
  pull_request:
    branches:
      - main

jobs:
  build:

    runs-on: ubuntu-latest

    strategy:
      matrix:
        rust: [stable, nightly]

    steps:
    - name: Check out repository code
      uses: actions/checkout@v2

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        override: true

    - name: Install dependencies
      run: sudo apt-get install libssl-dev

    - name: Cache cargo registry
      uses: actions/cache@v2
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}
        restore-keys: ${{ runner.os }}-cargo-registry-

    - name: Cache cargo index
      uses: actions/cache@v2
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}
        restore-keys: ${{ runner.os }}-cargo-git-

    - name: Build project
      run: cargo build --verbose

    - name: Run tests
      run: cargo test --verbose
```

I have not used GitHub Actions much.

I learn a lot when you ask questions in the repository I have created. Thank you.
