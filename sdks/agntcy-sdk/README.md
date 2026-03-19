# AGENTCY Rust Type Gen
This purpose of this repository is to generate a Rust SDK from the AGNTCY's OASF (json) schema

## Getting start

The generation process for this SDK is done in multiple containers. Run the following commands to generate the sdk:

`./manage init agntcy-sdk`

This will build the image required to generate the sdk

`./manage gen agntcy-sdk`

This does the following:
0. Pulls the specs from the OASF API
0. Generates the code using predefined templates
0. Overwrites previously generated cargo configs
0. Moves the code into the cargo package