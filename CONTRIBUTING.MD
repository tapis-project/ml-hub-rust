
# Contributing to Tapis ML Hub 🤠

Thank you for considering contributing to **Tapis ML Hub!** We welcome all kinds of contributions that help us build a robust and maintainable codebase.

Please take a few minutes to review this document so you can understand the standards we follow and help us keep the project consistent and high-quality.

---

## Getting Started

1. **Fork the repository** and clone it to your local machine.
2. **cd** into the project's root directory.
3. **Create a new branch** for your changes. We use the following naming conventions:
   - `feature/<feature-name>` for new features.
   - `bugfix/<issue-name>` for bug fixes.

---

## Code Style and Standards 😎

Please refer to the official [Rust style guide](https://doc.rust-lang.org/nightly/style-guide/) for general styling information. We follow these standards in the development of Tapis ML Hub and we would appreciate it you did too! All points below are to be used in addition to the standards from the aforementioned style guide.

1. **Linting**: We use ??? to enforce code style. Please ensure your code passes linting before creating a pull request (PR).
2. **Naming Conventions**: Use descriptive variable, function, macros, impls, and struct names. Keep them short, but clear.

---

## Design Patterns 🖼️

We use specific design patterns to ensure consistency across the project. Here are some of the main patterns and guidelines on when to use them:

- **Singleton Pattern**: Use this pattern for structs that should only have a single instance. Examples include configurations or database connection managers.
  
- **Factory Pattern**: Use this for complex object creation where the specific class of object created depends on runtime parameters. This pattern is particularly useful in [specific modules].

- **Repository Pattern**: Use this for data access layers to abstract the business logic from database operations.

> **Note**: When adding new code, refer to existing patterns and follow them whenever applicable. If your contribution requires a new pattern, please mention it in the PR description so we can review its suitability.

---

## Testing

1. **Unit Tests**: Add or update unit tests for any new features or bug fixes.
2. **Test Coverage**: Our goal is to maintain at least 80% test coverage across modules. If you can, please add coverage for any critical code paths.
3. **Run Tests**: Before submitting your PR, ensure all tests pass by running:

   ```bash
   ./test


## Pull Request (PR) Guidelines

When submitting a PR, ensure the following:
- **Title**: Use a clear and concise title for your PR.
- **Description**: Provide a detailed description of your changes. Reference any relevant issues (e.g., "Fixes #123").
- **Screenshots**: If your PR involves UI changes, include screenshots.
- **Checklist**: Before submitting your PR, check off the following:
   - [ ] My code follows the existing style guide
   - [ ] I have added tests for my changes
   - [ ] I have updated the documentation as necessary
   - [ ] I have run the tests and they are passing

## License

By contributing to this project, you agree that your contributions will be licensed under the project's license. For more details, see LICENSE.md in the project root directory.

   