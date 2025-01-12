# Claude CLI Development Guide

This guide provides information for developers who want to contribute to or modify the Claude CLI.

## Project Structure

```
claude-cli/
├── claude-common/      # Shared library code
├── claude/            # Main CLI application
├── claude-config/     # Configuration utility
├── tests/            # Test suite
└── packaging/        # Distribution packaging
```

## Development Setup

1. Install Rust and development tools:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add clippy rustfmt
```

2. Clone the repository:
```bash
git clone https://github.com/jarmez/claude-cli
cd claude-cli
```

3. Install development dependencies:
```bash
# For RPM packaging
sudo dnf install rpm-build
# For DEB packaging
sudo apt-get install build-essential debhelper
```

## Building

Development build:
```bash
cargo build
```

Release build:
```bash
cargo build --release
```

## Testing

Run all tests:
```bash
cargo test --all-features
```

Run specific test categories:
```bash
cargo test --test unit_*    # Unit tests
cargo test --test int_*     # Integration tests
cargo test --test mock_*    # Mock tests
cargo test --test prop_*    # Property tests
```

Generate test coverage:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## Code Style

Format code:
```bash
cargo fmt
```

Run linter:
```bash
cargo clippy
```

## Creating Releases

1. Update version in Cargo.toml files
2. Update CHANGELOG.md
3. Create and push tag:
```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

## Package Building

### RPM Package
```bash
cd packaging/rpm
rpmbuild -ba claude-cli.spec
```

### DEB Package
```bash
cd packaging/debian
debuild -us -uc
```

### Homebrew Formula
```bash
cd packaging/homebrew
# Update SHA256 in formula
brew install --build-from-source ./claude-cli.rb
```

## MCP Server Integration

MCP servers are configured in `~/.config/claude-cli/mcp_servers.json`:

```json
{
  "servers": [
    {
      "name": "example",
      "url": "https://example.com/mcp",
      "api_version": "1.0",
      "tools": [
        {
          "name": "example_tool",
          "description": "Example tool description",
          "parameters": {
            "param1": {
              "type_name": "string",
              "description": "Parameter description",
              "required": true
            }
          }
        }
      ]
    }
  ]
}
```

## Architecture

### Core Components

1. Configuration Management
   - Config file handling
   - Environment variables
   - Lua/Vim config support

2. API Integration
   - Claude API client
   - Response handling
   - Error management

3. REPL Implementation
   - Command parsing
   - Session management
   - History handling

4. MCP Integration
   - Server configuration
   - Tool management
   - Request handling

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes
4. Add tests
5. Submit pull request

Please ensure:
- All tests pass
- Code is formatted
- Documentation is updated
- Changelog is updated

## Debugging

Enable debug logging:
```bash
RUST_LOG=debug claude
```

Use logging in code:
```rust
tracing::debug!("Debug message");
tracing::info!("Info message");
tracing::error!("Error message");
```