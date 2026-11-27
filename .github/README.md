# starter-rs

A [cargo-generate](https://github.com/cargo-generate/cargo-generate) template
for Rust CLI crates. It provides:

- clap and color-eyre, and nothing else, because Cargo warns on unused
  dependencies
- strict Clippy lints (`pedantic`, `nursery`, and no panicking shortcuts)
- a pinned toolchain in `rust-toolchain.toml` and pinned tools in `mise.toml`
- a justfile whose `just check` matches CI
- CI for formatting, Clippy, nextest, docs, and `cargo audit`
- a tag-driven release that builds Linux and macOS binaries, publishes to
  crates.io through trusted publishing, and creates a GitHub release with
  notes from git-cliff
- a git-cliff config that builds `CHANGELOG.md` from Conventional Commits

## Usage

```sh
cargo generate kristoferssolo/starter-rs --name my-tool
cd my-tool
just setup
just check
```

cargo-generate prompts for a description and the GitHub owner. `authors` comes
from your Git config; override it with `--define 'authors=Name <email>'`.

Commit `Cargo.lock` with the first commit. CI runs `cargo fetch --locked` and
fails without it.

## Releasing a generated project

1. Configure trusted publishing for the crate on crates.io, pointing at
   `publish.yml`.
2. Bump `version` in `Cargo.toml`, run `just changelog --tag vX.Y.Z`, commit,
   and push a matching `vX.Y.Z` tag.

## Developing the template

Only the files listed in `include` in `cargo-generate.toml` go through Liquid.
The justfile, workflows, and `cliff.toml` use `{{ }}` for their own syntax, so
keep Liquid out of them; `publish.yml` reads the crate name from
`cargo metadata` instead.

The `Template` workflow generates a project and runs its `just check`. The CI
and release jobs skip themselves while the repository is marked as a GitHub
template. To run the smoke test locally:

```sh
cargo generate --path . --name smoke-test --destination "$TMPDIR" --silent \
    --define 'description=Template smoke test' --define github_user=example
```

After changing tool versions in `mise.toml`, run `mise lock` to refresh
`mise.lock`.
