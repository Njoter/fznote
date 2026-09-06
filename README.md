fznote - Fuzzy Note Manager
---------------------------

fznote is a lightweight, terminal-based note manager that combines the power
of fzf with the simplicity of plain text files. Write your notes in any editor,
find them instantly with fuzzy search, and version control them with git.

**Early Development**: fznote is actively evolving. Features may change,
and some functionality is still in progress. Contributions and feedback are
welcome!

## Features

    🎯 Fuzzy search - Find any note instantly with fzf

    📝 Plain text - Your notes are just files. Use any editor, any format

    🔒 Git-backed - Version control built in (coming soon)

    🎨 Preview and printing with configurable reader

    🔍 Content search - Search through all your notes with ripgrep + fzf

    🔌 Terminal-native - Works in your workflow, not against it

    ⚙️ Configurable - Customize reader, editor, and more

## Installation
    bash
    git clone https://github.com/Njoter/fznote
    cd fznote
    cargo install --path .

## Requirements
*   fzf - The heart of fznote. Required for fuzzy finding.
*   Some kind of reader. I recommend bat (or mdcat if you like markdown).
*   ripgrep - Optional for content search

## Basic commands
    # Default: print a note to the terminal with cat
    fznote

    # Read a note with your configured reader (bat, less, mdcat)
    fznote read

    # Open a note in your configured editor
    fznote edit

    # Add a new note
    fznote add mynote

    # Add with custom extension
    fznote add mynote -x txt

    # Delete a note
    fznote delete

    # Print the full path of a note (great for scripting)
    fznote path

    # Search content and read
    fznote read -s

    # Search content and edit
    fznote edit -s

    # Search content and delete
    fznote delete -s

## The -s Flag
The ```-s``` (search) flag transform any command into content search, letting
you search through the content of all your notes before selecting the one you need.

## Configuration
fznote uses a YAML config file at ~/.config/fznote/config.yaml

### Default configuration
    directory: /home/user/.local/share/fznote/notes
    reader: bat
    preview_reader: bat
    editor: vim
    file_extension: md
