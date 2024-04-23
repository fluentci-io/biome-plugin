# Biome Plugin

[![ci](https://github.com/fluentci-io/biome-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/biome-plugin/actions/workflows/ci.yml)

This plugin sets up your CI/CD pipeline with a specific version of [biome](https://biomejs.dev/).

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm biome setup 1.7.0
```

## Functions

| Name   | Description                                                       |
| ------ | ----------------------------------------------------------------- |
| setup  | Installs a specific version of biome.                             |
| format | Run the formatter on a set of files.                              |
| lint   | Run various checks on a set of files.                             |
| ci     | Runs formatter, linter and import sorting to the requested files  |
| check  | Runs formatter, linter and import sorting to the requested files. |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.1.9"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/biome@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: biome
    args: |
      setup
- name: Show Biome version
  run: |
    export PATH=${HOME}/.bun/bin:${PATH}
    type biome
    biome --version
```
