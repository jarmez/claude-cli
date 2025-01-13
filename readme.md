# Claude CLI
* Currently in testing, not ready for general use, you are welcome to fork, compile and test, and provide feedback.
* Currently working on integating MCP server configuration for custom extensibility.

A command-line interface for the Claude AI assistant with Vim/Neovim-like interaction.


## Features

- Interactive REPL with Vim-style commands
- Single command execution mode
- Multiple output formats (txt, json, csv)
- Vim/Neovim friendly interface
- Configurable via Lua or vimscript
- Session management and history
- Comprehensive logging

## Installation

```bash
cargo install claude-cli
```

## Environment Variables

- `CLAUDE_API_KEY`: Your Claude API key
- `CLAUDE_MODEL`: Override default model (default: claude-3-sonnet)
- `CLAUDE_CONFIG_DIR`: Override default config directory (default: ~/.config/claude-cli)
- `CLAUDE_LOG_LEVEL`: Set logging level (default: info)
- `CLAUDE_OUTPUT_FORMAT`: Default output format (default: text)

## Usage

### Single Command Mode
```bash
# Simple query
claude "What is the capital of France?"

# Specify model
claude -m claude-3-opus "Complex analysis task"

# Output as JSON
claude --format json "Generate a list of cities"
```

### Interactive Mode
Start an interactive session:
```bash
claude
```

In interactive mode, use Vim-style commands:
```
<Esc>:help           # Show help
<Esc>:q             # Quit
<Esc>:model opus    # Switch to Claude-3 Opus
<Esc>:save proj1    # Save session
<Esc>:list          # List history
```

## Configuration

Default configuration locations:
- Claude CLI: `~/.config/claude-cli/config.json`

## Logging

Logs are stored in `~/.config/claude-cli/logs/`.

## Development

### Building from Source
```bash
git clone https://github.com/jarmez/claude-cli
cd claude-cli
cargo build --release
```

## License

MIT License
