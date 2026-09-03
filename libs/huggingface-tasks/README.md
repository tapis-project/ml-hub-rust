# Huggingface Task Code Generator and Library

## NEW TASKS URL https://huggingface.co/api/models-tags-by-type

This library generates a Rust enum for each task type available in Huggingface.

For the following steps, run the code snippets from the root of the project

0. Build the images that will handle the code generation and cargo crate initialization
    - `./dev init huggingface-tasks`

0. Generate the code. This will run in two steps, each in their own container.
    - `./dev gen huggingface-tasks`