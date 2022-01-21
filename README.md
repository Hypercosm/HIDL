# Hypercosm Interface Description Language (HIDL)

This reposity (will) contains 4 things:

- A specification for HIDL
- HIDL files for the core hypercosm protocol
- Rust tools for working with HIDL
- Generated output of the tool

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