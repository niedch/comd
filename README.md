# comd

A terminal UI that streams AI responses from Google's Gemini API directly into your command line.

## Features

- Inline TUI for entering prompts and viewing responses
- Streaming AI responses in real-time
- Gemini 2.5 Flash integration
- Zsh integration for shell command generation (writes output to a buffer file)

## Installation

```bash
cargo install comd
```

## Configuration

Copy `config/config.toml` to one of these locations:
- `./comd.toml`
- `$COMD_CONFIG.toml`
- `$HOME/.config/comd/comd.toml`

Edit the config and add your Gemini API key:

```toml
[global]
system_prompt = """
You are a helper Bot for Bash! Only responded with a single line of bash. Only bash! No Backticks!
"""
gemini_api_key = "your-api-key-here"
model = "gemini-2.5-flash"
```

Get your API key from [Google AI Studio](https://aistudio.google.com/app/apikey).

## Usage

Run the application:

```bash
comd
```

Type your prompt and press Enter to submit. The AI response will stream in real-time.

### Keybindings

- `Enter` - Submit prompt
- `Arrow Left/Right` - Move cursor
- `Backspace` - Delete character
- `Esc` - Exit

### Zsh Integration

To use with zsh, set the `COMD_ZSH_BUFFER_FILE` environment variable before running:

```bash
COMD_ZSH_BUFFER_FILE=/tmp/comd-output comd
```

After running, the output will be written to the specified file instead of printed to stdout.