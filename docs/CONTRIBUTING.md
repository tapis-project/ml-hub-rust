# Contributing to MLHub 🤠

Thank you for considering contributing to **MLHub!**. We welcome all kinds of contributions that help us build a robust and maintainable codebase.

Please take a few minutes to review this document so you can understand the standards we follow and help us keep the project patterns and styles consistent and high-quality.

Before contributing to this project, please read this document as well as the [DEVELOPERS GUIDE](./DEVELOPERS_GUIDE.md) for a more in-depth explanation of this project's components, directory structure, and architecture.

For the complete feature-development workflow, follow the [API Development Playbook](./API_DEVELOPMENT_PLAYBOOK.md). Contributors using AI coding agents should ensure those agents also follow the repository-root [AGENTS.md](../AGENTS.md) and the [AI Agent Development Guide](./AI_AGENT_DEVELOPMENT_GUIDE.md).

---

## Getting Started

1. **Fork the repository** and clone it to your local machine.
2. **cd** into the project's root directory.
3. **Create a new branch** for your changes. We use the following naming conventions:
   - `feature/<feature-name>` for new features.
   - `bugfix/<issue-name>` for bug fixes.

---

## Pull Request (PR) Guidelines

When submitting a PR, ensure the following:
- The number of modified files should be around 5-10 files. up to ~20 files is acceptable for a major refactor. More than ~20 files will require a substantial explanation detailing why such a large change needs to be made.
- **Title**: Use a clear and concise title for your PR.
- **Description**: Provide a detailed description of your changes. Reference any relevant issues (e.g., "Fixes #123").
- **Screenshots**: If your PR involves UI changes, include screenshots.
- **Checklist**: Before submitting your PR, check off the following:
   - [ ] My code follows the existing style guide
   - [ ] I have added tests for my changes
   - [ ] I have updated the documentation as necessary
   - [ ] I have run the tests and they are passing
   - [ ] I have validated that the new feature(s) I added work as expected
   - [ ] I did not break any existing functionality

## Attribution

To ensure you are recognized for your contributions to ML Hub, please add a link to your Github (or equivalent) profile to the CONTRIBUTORS.md file in the projects root directory.

## Code Style and Standards 😎

Please refer to the official [Rust style guide](https://doc.rust-lang.org/nightly/style-guide/) for general styling information. We follow these standards in the development of MLHub and we would appreciate it you did too! All points below are to be used in addition to the standards from the aforementioned style guide.

1. **Linting**: We use ??? to enforce code style. Please ensure your code passes linting before creating a pull request (PR).
2. **Naming Conventions**: Use descriptive variable, function, macros, impls, and struct names. Keep them short, but clear.

---

## Design Patterns 🖼️

We use specific design patterns to ensure consistency across the project. Here are some of the main patterns and guidelines on when to use them:

- **Singleton Pattern**: Use this pattern for structs that should only have a single instance. Examples include configurations or database connection managers.
  
- **Factory Pattern**: Use this for encapsulating logic for creating complex or polymorphic objects.

- **Repository Pattern**: Use this for data access layers to insulate the business logic from persistence-related infrastructure concerns.

> **Note** See Service Pattern below

- **Service Pattern** Use this pattern to encapsulate complex business logic including DTO validation, interactions with the Data Layer via repositories (mentioned above), orchestrating application and domain logic with cross-cutting concerns, etc.

> **Note**: When adding new code, refer to existing patterns and follow them whenever applicable. If your contribution requires a new pattern, please mention it in the PR description so we can review its suitability for this project and add it to this documentation.

---

## License

By contributing to this project, you agree that your contributions will be licensed under the project's license. For more details, see LICENSE.md in the project root directory.
