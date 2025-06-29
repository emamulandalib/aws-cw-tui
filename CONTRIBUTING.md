# Contributing to AWS CloudWatch TUI

Thank you for your interest in contributing to AWS CloudWatch TUI! This document outlines the process for contributing to this project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Making Changes](#making-changes)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Reporting Issues](#reporting-issues)
- [Community](#community)

## Code of Conduct

This project adheres to a code of conduct that we expect all contributors to follow. Please be respectful and constructive in all interactions.

## Getting Started

### Prerequisites

- Rust 1.70 or higher
- Git
- AWS CLI (optional, for testing)

### Fork and Clone

1. Fork the repository on GitHub
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR_USERNAME/aws-cw-tui.git
   cd aws-cw-tui
   ```
3. Add the upstream repository:
   ```bash
   git remote add upstream https://github.com/emamulandalib/aws-cw-tui.git
   ```

## Development Setup

1. **Install dependencies:**
   ```bash
   cargo build
   ```

2. **Run the application:**
   ```bash
   cargo run -- --rds
   ```

3. **Run tests:**
   ```bash
   cargo test
   ```

4. **Check code formatting:**
   ```bash
   cargo fmt --check
   ```

5. **Run linter:**
   ```bash
   cargo clippy -- -D warnings
   ```

## Making Changes

### Branch Naming

Use descriptive branch names with prefixes:
- `feat/` for new features
- `fix/` for bug fixes
- `docs/` for documentation changes
- `refactor/` for code refactoring
- `test/` for adding tests

Example: `feat/add-ec2-monitoring`

### Commit Messages

Follow the Conventional Commits specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that do not affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools

**Examples:**
```
feat: add EC2 instance monitoring support

Add comprehensive EC2 metrics collection including CPU, memory,
network, and disk utilization with real-time visualization.

Closes #123
```

```
fix: resolve memory leak in metric collection

The metric collection loop was not properly cleaning up resources
leading to memory growth over time.

Fixes #456
```

### Code Style

- Follow Rust conventions and use `rustfmt`
- Write clear, self-documenting code
- Add comments for complex logic
- Use meaningful variable and function names
- Keep functions small and focused

### Architecture Guidelines

- Follow the existing project structure
- Use the established error handling patterns (`anyhow::Result`)
- Implement proper async/await patterns with tokio
- Follow the modular architecture for AWS services
- Use dependency injection for testability

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Writing Tests

- Write unit tests for all new functionality
- Use integration tests for AWS service interactions
- Mock external dependencies where appropriate
- Test error conditions and edge cases
- Aim for high test coverage

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_metric_collection() {
        // Arrange
        let client = MockCloudWatchClient::new();
        
        // Act
        let result = collect_metrics(&client).await;
        
        // Assert
        assert!(result.is_ok());
    }
}
```

## Submitting Changes

### Pull Request Process

1. **Update your fork:**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Create a pull request:**
   - Use a descriptive title
   - Reference related issues
   - Provide a clear description of changes
   - Include screenshots for UI changes
   - Add testing instructions

3. **Pull request template:**
   ```markdown
   ## Description
   Brief description of changes

   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature  
   - [ ] Breaking change
   - [ ] Documentation update

   ## Testing
   - [ ] Tests pass locally
   - [ ] Added/updated tests
   - [ ] Manual testing completed

   ## Checklist
   - [ ] Code follows project style guidelines
   - [ ] Self-review completed
   - [ ] Documentation updated
   ```

### Review Process

- All PRs require review from maintainers
- Address feedback promptly
- Keep PRs focused and atomic
- Be responsive to comments and suggestions

## Reporting Issues

### Bug Reports

Use the bug report template and include:
- Clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Environment details (OS, Rust version, AWS region)
- Relevant logs or error messages

### Feature Requests

Use the feature request template and include:
- Clear description of the desired feature
- Use case and motivation
- Proposed implementation approach
- Alternatives considered

### Security Issues

For security vulnerabilities, please email security@aws-cw-tui.dev instead of opening a public issue.

## Community

### Communication

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: Questions and general discussion
- **Pull Requests**: Code review and collaboration

### Getting Help

- Check existing issues and documentation
- Ask questions in GitHub Discussions
- Review the README and code examples

## Recognition

Contributors will be recognized in the project's acknowledgments. Significant contributors may be invited to become maintainers.

## License

By contributing to this project, you agree that your contributions will be licensed under the same license as the project (MIT License).

---

Thank you for contributing to AWS CloudWatch TUI!