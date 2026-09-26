# TMF Generator

A code generator for tmflib.

## Description

This utility takes a set of TMF OpenAPI specification files as input and
generates corresponding Rust output code for use within the tmflib
ecosystem.

Generated modules are **committed** into tmflib's `src/` tree as plain
source files. Generation does not run at build time; it is a manual,
on-demand step whenever a spec or the generator changes.

## Usage

Regenerate all configured modules into tmflib's `src/` tree:

```sh
cargo run -p tmf-gen -- --all --output src
```

Generate a single spec to an arbitrary folder:

```sh
cargo run -p tmf-gen -- --file open_api/TMF628_Performance-v5.0.0.oas.yaml \
    --output /tmp/out --tmf tmf628
```

Modules are configured in `tmf-gen/src/lib.rs` (`config_for_feature`);
a module only participates in `--all` once it has a configuration there.

## Workflow

1. Add or update a spec in `open_api/`.
2. Add a module configuration (`spec_file`, `tmf_module`, shared types,
   managed types) via `config_for_feature`.
3. Run `cargo run -p tmf-gen -- --all --output src`.
4. Wire the module into `src/lib.rs` (`#[cfg(feature = "tmfNNN")] pub mod tmfNNN;`).
5. Commit the generated `src/tmfNNN.rs` together with any spec changes.

The `drift` test in `tmf-gen/tests/` fails if a committed generated module
no longer matches its spec, so regeneration can be enforced in CI.

## Spec handling

* OpenAPI 3.x and Swagger 2.0 documents are both supported; only the
  schema definitions (`components.schemas` / `definitions`) are used.
* Spec-quality issues are recovered automatically and reported as
  `// WARNING:` comments in the generated output:
  * `$ref` targets that are never defined get empty placeholder schemas.
  * Schema `default` values are dropped (tmflib does not use them; TMF
    specs contain several invalid ones, e.g. string defaults on booleans).
* Base types that tmflib provides in `common/` (e.g. `Extensible`,
  `Entity`, `TimePeriod`) are not generated; references are rewritten to
  the crate types and `allOf` compositions become `#[serde(flatten)]`
  fields.
* "Managed" types (configured per module) receive `CLASS_PATH` consts and
  `HasId`/`HasDescription` derives for TMF URL generation.
