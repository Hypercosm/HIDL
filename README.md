# Hypercosm Interface Description Language (HIDL)

⚠️ **extremely serious warning:** this is _pre-alpha_, proof-of-concept
software! everything here has _no stability guarantees_ &mdash;
the HIDL language, the contents of the HIDL files, and the JSON representation
in this repository are not guaranteed to be interoperable except
within the same Git revision.

This reposity (will) contains 4 things:

- [A specification for HIDL](HIDL.md)
- [HIDL files for the core hypercosm protocol](proto)
- [Rust tools for working with HIDL](src)
- Generated outputs ([json](json), [docs](proto-docs)) of the tool

## Running the Tools

Currently their are two tools shiped in this repo, a json generator
and a documentation generator. Both of them require the latest stable
`cargo` to be installed.

- `cargo run --bin generate -- hypercosm.hidl json/hypercosm.json` to generate a json
  desciption of the API
- `cargo run --bin document --  hypercosm.hidl proto-docs` to generate markdown documentation
  for the API

Any time you modify a `hidl` file or the tools, you should run both of these commands. One day
CI will enforce this.