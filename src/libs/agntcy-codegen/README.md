# AGENTCY Rust Type Gen
This purpose of this repository is to generate Rust types from the AGNTCY's OASF (json) schema

As with many (most) efforts at generating types from some formal spec to any language, this
project requires a bit of pre processing of the spec and postprocessing of the generated types

## Getting start
The `generate` script below will take you through the following steps:
0. Preprocess the spec (manual)
0. Dereference the spec (automatic)
0. Generate the Rust types (automatic)

Run the following command

`chmod +x generate`

`./generate`
