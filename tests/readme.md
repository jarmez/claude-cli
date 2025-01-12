# Testing Claude CLI

This directory contains different types of tests for the Claude CLI application.

## Test Types

### Unit Tests
Located in `tests/unit/`. These test individual components in isolation.
```bash
cargo test --lib
```

### Integration Tests
Located in `tests/integration/`. These test the CLI as a whole.
```bash
cargo test --test '*'
```

### Mock Tests
Located in `tests/mocks/`. These simulate API interactions.
```bash
cargo test --test mock_*
```

### Property Tests
Located in `tests/property/`. These test data handling with random inputs.
```bash
cargo test --test prop_*
```

## Running Tests

Run all tests:
```bash
cargo test
```

Run specific test categories:
```bash
cargo test --test unit_*    # Run unit tests
cargo test --test int_*     # Run integration tests
cargo test --test mock_*    # Run mock tests
cargo test --test prop_*    # Run property tests
```

## Test Coverage

Generate test coverage report:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Adding New Tests

### Unit Tests
Add new unit test files in `tests/unit/`:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

### Integration Tests
Add new integration test files in `tests/integration/`:
```rust
use assert_cmd::Command;

#[test]
fn test_cli_command() {
    let mut cmd = Command::cargo_bin("claude").unwrap();
    cmd.arg("--help")
        .assert()
        .success();
}
```

### Mock Tests
Add new mock test files in `tests/mocks/`:
```rust
use mockall::automock;

#[automock]
trait ClaudeApi {
    fn chat(&self, message: &str) -> Result<String>;
}
```

### Property Tests
Add new property test files in `tests/property/`:
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_config_serialization(s in ".*") {
        // Property test implementation
    }
}
```