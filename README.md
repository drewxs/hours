# Hours

Simple hours tracking CLI.

## Installation

```sh
cargo install --git https://github.com/drewxs/hours
```

## Usage

```sh
# Show help
hours -h

# Add hours to a project (creates a new project if it doesn't exist)
hours -p <project_name> -n <hours>
# e.g.
hours -p foo -n 1.5

# List all projects
hours -l

# Start a session
# This creates a timestamp, then adds when the session is ended
hours -sp <project_name>

# End current session
hours -e

# Clear
hours -c
```
