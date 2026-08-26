# MLHub Agent Instructions

These instructions apply throughout this repository.

## Start with established patterns

- Inspect the closest Models and Deployments implementation before adding a service, layer, endpoint, DTO, repository, bootstrap component, or deployment manifest. Follow the established pattern exactly unless a change is explicitly requested.
- Preserve unrelated working-tree changes. Do not reset, restore, reformat, or regenerate broad areas of the repository to complete a focused task.
- Make a small, coherent change and validate the affected packages and manifests before handoff.

## Architecture and placement

- MLHub uses presentation, application, domain, infrastructure, and bootstrap layers. Reusable implementations belong in `libs/shared`; services re-export the shared modules they consume rather than duplicating them.
- Keep HTTP handlers and route registration service-local. Give every handler its own file and register handlers following the Models and Deployments pattern; do not introduce a route-configuration function where those services do not use one.
- Put each mapper in its own file. Prefer `From` for infallible conversion and `TryFrom` for fallible conversion over named mapping functions.
- Use a directory with `mod.rs` when a component has adjacent tests or supporting files. A component with no tests or support files may remain in a same-named file.

## Domain and application rules

- Keep domain state private. Construct new entities with `new`, rehydrate persisted data with props-based `reconstitute`, and use UUID v7 for new identities where the established entity pattern does so.
- Enforce invariants in the domain. New input should receive a specific domain error; invalid persisted data should receive a data-integrity error. Use structural types such as `NonEmpty` where the domain requires them.
- Prefer semantic domain queries over exposing nested value objects. When a new value object needs an access interface and the desired encapsulation is unclear, ask before choosing it.
- Application-service methods take `&RequestContext` as their first argument. Derive tenancy and principal information from that context, not HTTP input.
- Infrastructure uses its own persistence document DTOs and conversions. Do not query or persist enum/string storage literals directly when a document DTO exists.

## Presentation and API rules

- Put request/response DTOs and OpenAPI contracts in shared presentation modules; use validation at the request boundary to fail fast while retaining domain validation as authoritative.
- Use lower_snake_case for field names. Preserve Rust enum variant casing on the wire unless the API explicitly requires a transformation.
- Document handlers with Utoipa. Generated OpenAPI is source-derived: do not manually edit checked-in generated specifications unless explicitly requested.
- Apply the established authentication, tenancy, logging, and CORS/preflight middleware. Keep explicitly public routes public.

## Tests, formatting, and delivery

- Co-locate tests as adjacent `*.test.rs` files and include them through the module convention used by nearby code. Use builders with optional fields for entity tests. Return and handle errors explicitly in tests; do not use `unwrap` for expected success or failure paths.
- Keep a blank line between methods, logical statements, and code blocks; imports stay scoped at the top of a file. Alias imports rather than using inline fully qualified paths.
- For a long method chain outside a closure or iterator adapter, put each chained call on its own continuation line. Leave a blank line after a completed fallible operation before beginning the next logical operation.
- Run focused `cargo test`, `cargo check`, formatting checks, Kustomize rendering, and `git diff --check` in proportion to the change. Never expose credentials, tokens, or secrets in source, documentation, commands, or handoff messages.
