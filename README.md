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

# Start a session
# This creates a timestamp, then adds when the session is ended
hours session start <PROJECT>

# Switch session to a different project
hours session switch <PROJECT>

# End current session
hours session end

# View current session
hours session view

# Clear
hours clear
```
