# WebPublish

Schema-driven Cloudflare Pages apply engine.

## Command surface

`webpublish apply` is the single entry point. The command accepts no arguments,
reads a single packed Cap’n Proto `WebPublishConfiguration` message from stdin,
and emits `wrangler` stdout and stderr verbatim.

## Execution model

The apply engine performs two reconciliations:

1. Ensure a Cloudflare Pages project exists.
2. Ensure custom domain bindings exist.

Idempotency is defined by `wrangler` output. A zero exit code or recognized
“already exists/added/bound” output counts as success. Any other non-zero exit
terminates apply with a non-zero exit code.

## Schema-to-effect mapping

The `webpublish.capnp` schema defines the `WebPublishConfiguration` object. The
apply engine maps fields to the `wrangler` CLI:

- `source.owner` + `source.repository` → `<owner>/<repository>` GitHub source.
- `pages.projectName` → Pages project identifier.
- `pages.productionBranch` → production branch selector.
- `pages.buildCommand` → `--build-command` (opaque pass-through).
- `pages.buildOutputDirectory` → `--build-output` (opaque pass-through).
- `pages.accountId` → `--account-id` flag when supported.
- `domains.primaryDomain` + `domains.alternateDomains` → `wrangler pages domain add`.

## Wrangler argv shapes

Project phase:

```sh
wrangler pages project create <projectName> \
  --production-branch=<productionBranch> \
  --source=github \
  --repo=<owner>/<repository> \
  --build-command=<buildCommand> \
  --build-output=<buildOutputDirectory>
```

Domain phase:

```sh
wrangler pages domain add <projectName> <domain>
```

## Build

The workspace contains two crates:

- `crates/webpublish` provides the library objects.
- `bin/webpublish` provides the binary entry point.

The Cap’n Proto schema is compiled at build time using `capnpc`.
