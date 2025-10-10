# AGENTCY Rust Type Gen
This purpose of this repository is to generate a Rust SDK from the AGNTCY's OASF (json) schema

As with many (most) efforts at generating types from some formal spec to any language, this
project requires a bit of pre processing of the spec and postprocessing of the generated types

## Getting start

The generation process for this SDK is done in a container.

0. Build the container: `./manage build agntcy-sdk`
0. Run the container interactively: `./manage run agntcy-sdk`
0. Once in the container, run the **generate** script: `./generate`

0. Preprocess the spec (manual)
0. Dereference the spec (automatic)
0. Generate the Rust types (automatic)