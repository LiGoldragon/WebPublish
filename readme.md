# WebPublish
Typed web publication daemon using Cap’n Proto + Rust

WebPublish is a Rust daemon that receives Cap’n Proto `WebPublishConfiguration`
messages and performs deterministic hosting operations for websites. The initial
hosting backend is Cloudflare Pages, and the architecture is designed for
seamless extension to additional providers.

## Goals

- Treat the Cap’n Proto schema as the single source of truth for site
  configuration.
- Maintain strong Rust typing end-to-end, with minimal glue.
- Support continuous, streamed provisioning via stdin.
- Integrate cleanly with Nix for build, packaging, and deployment.
- Keep hosting backends modular and replaceable.

## High-level architecture

1. Upstream tools emit `WebPublishConfiguration` messages using the
   `webpublish.capnp` schema.
2. Messages are serialized in **packed Cap’n Proto** format.
3. `webpublishd` reads a stream of messages from stdin.
4. Each message is decoded into a typed Rust `WebPublishConfiguration`.
5. A hosting backend is selected (e.g. Cloudflare Pages).
6. The backend ensures that projects and domains are in the requested state.

## Cap’n Proto schema

The schema is the authoritative definition of configuration. A minimal example:

```capnp
@0xbad0bad0bad0bad0;

using Rust = import "rust.capnp";

$Rust.module("webpublish_capnp");

struct Site {
  id    @0 :Text;
  title @1 :Text;
}

struct WebPublishConfiguration {
  site @0 :Site;
}
```

In the actual project, the schema also defines:

- `SiteIntent`, `HostingAuthorityRole`
- `DeploymentArtifact`
- `DomainAssignment`, `NameResolutionConfiguration`
- The full `WebPublishConfiguration` tree

Rust types for these are generated at build time by `capnpc-rust`.

## Rust domain model

The daemon converts Cap’n Proto readers into a small domain model used by the
hosting logic. Example:

```rust
#[derive(Debug, Clone)]
pub struct Site {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone)]
pub struct WebPublishConfiguration {
    pub site: Site,
    // artifact, domains, hosting, name resolution, acquisition, ...
}
```

The mapping is centralized in `decode.rs`, using functions that convert from
`webpublish_capnp::web_publish_configuration::Reader<'_>` into
`WebPublishConfiguration`.

## Nix integration

The project is designed to be managed primarily via Nix flakes.

### Build with Nix

```sh
nix build .#webpublish
```

This produces a `result` symlink with the `webpublishd` binary.

### Run with Nix

```sh
# Single configuration message from a file
nix run .#webpublish -- --help
nix run .#webpublish -- < config.bin

# Stream of messages from another process
produce-publications | nix run .#webpublish --
```

### Nix development environment

```sh
# Drop into a dev shell with rustc, cargo, capnp, etc.
nix develop
```

The Nix dev shell also bundles `rust-analyzer`, so editors using Eglot or
`lsp-mode` can offer completions and diagnostics without additional setup.

Inside the dev shell:

```sh
cargo build
cargo test
```

Nix can also provide `wrangler` and other runtime tools required by specific
hosting backends.

## Development environment

This repository provides a flake-based dev shell.

- To enter the shell manually:
  - `nix develop` (in the project root)

- With `direnv` + `nix-direnv`:
  - Install `direnv` and `nix-direnv`.
  - Run `direnv allow` once in the project root.
  - The environment will auto-activate when entering the directory.

- With Emacs `envrc`:
  - Install the `envrc` package.
  - Run `direnv allow` once in the project root so the environment can be
    loaded.
  - Enable `envrc-mode` in project buffers to load the environment from
    `.envrc`.

## Building without Nix

Nix is preferred but not strictly required.

```sh
# Install the Cap’n Proto compiler
# Debian/Ubuntu:
sudo apt install capnproto

# macOS (Homebrew):
brew install capnp

# Then build with Cargo
cargo build
```

## Running the daemon

The daemon reads one or more packed Cap’n Proto `WebPublishConfiguration`
messages from stdin. For example:

```sh
# Single message from a file
webpublishd < config.bin
```

```sh
# Stream of messages
produce-publications | webpublishd
```

Each message is processed in sequence. Hosting changes are applied idempotently
where possible.

## Environment variables (Cloudflare Pages backend)

The Cloudflare Pages backend expects the following environment variables to be
set:

```sh
export CLOUDFLARE_API_TOKEN=...
export CLOUDFLARE_ACCOUNT_ID=...
```

The daemon will exit with an error if these are missing when the Cloudflare
backend is in use.

## Hosting backends

Backends live under `src/hosting/` and are selected based on the
`HostingAuthorityRole` enum in the incoming configuration. Each backend applies
hosting and domain binding instructions deterministically to match the
configuration supplied in the publication message.
