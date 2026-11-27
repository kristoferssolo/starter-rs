# {{project-name}}

{{description}}

## Installation

```sh
cargo install --locked {{project-name}}
```

Prebuilt binaries for Linux and macOS are attached to each
[release](https://github.com/{{github_user}}/{{project-name}}/releases).

## Development

```sh
just setup  # install the Rust toolchain and the tools pinned in mise.toml
just check  # formatting, Clippy, tests, and docs, as CI runs them
just        # list all recipes
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or
[MIT license](LICENSE-MIT) at your option.
