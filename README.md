# Hours

Simple hours tracking CLI.

## Installation

```sh
cargo install --git https://github.com/drewxs/hours
```

## Usage

```sh
# Show help
hours help

# List all projects/hours
hours list

# Add hours to a project (creates a new project if it doesn't exist)
hours add <PROJECT> <HOURS>

# Switch session to a different project
hours switch <PROJECT>

# Start a session
# This creates a timestamp, then adds when the session is ended
hours start <PROJECT>

# End current session
hours end

# Clear
hours clear
```
