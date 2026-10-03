fznote - Fuzzy Note Manager
===========================

A fuzzy note manager powered by fzf.

fznote manages plain text notes in a directory of your choosing, synced
with git. Notes are files; books are directories; sync is git. fznote
doesn't try to be a note-taking application — it's a thin layer that
orchestrates the tools you already use into a fast terminal workflow.
Your notes stay yours, in a format you can read without fznote, and the
tool stays out of the way.

Works on Linux and Windows.

Features
--------

* Fuzzy note selection with fzf
* Search through the contents of all your notes with ripgrep
* Books (directories) for organizing notes
* Git sync with a single command
* Plain Markdown files, editable with any editor

Requirements
------------

**Required**
* fzf - the heart of fznote
* git - for sync

**Recommended**
* bat - default reader, for fzf previews and `fznote read`
* nvim - default editor for writing the notes
* ripgrep - for `-s` (content search)

The reader and editor can be set in the config if you don't want to use the defaults.
`ripgrep` is optional, but the `-s` flag will fail without it.

Installation
------------

```bash
git clone https://github.com/Njoter/fznote
cd fznote
cargo install --path .
```

Basic commands
--------------

```bash
# Default: print a note to the terminal
fznote [-s]

# Read a note with your configured reader (bat, less, mdcat)
fznote read [-s]

# Open a note in your configured editor
fznote edit [-s]

# Add a new note
fznote add mynote

# Add with custom extension
fznote add mynote -x txt

# Delete a note
fznote delete [-s]

# Print the full path of a note
fznote path [-s]
```

The -s Flag
-----------

The `-s` (search) flag transforms a command into content search, letting
you select a note by what's inside it, instead of by its name.

Books
-----

Books are directories inside the notes root. `fznote books list` lists them, and the current book is marked.
The current book is where all the basic commands operate.

```bash
# Default: Select a book with fzf and switch to it
fznote books

# Create a book
fznote books add "New Book"

# Rename a book
fznote books rename

# Delete a book
fznote books delete

# List your books
fznote books list
```

Sync
----

Sync is git. fznote sync fetches, commits any uncommitted changes,
pulls if behind, and pushes if ahead. One command, no manual git.

```bash
# Configure the remote (first time only)
fznote sync setup

# Reconfigure the remote
fznote sync setup --force

# Sync with the remote
fznote sync
```

The notes directory is a git repo. You can cd into it and use git
directly if you want — fznote doesn't hide it.

Configuration
-------------

Config lives at `~/.config/fznote/config.yaml` (Linux) or `%APPDATA%\fznote\config.yaml` (Windows).

### Default configuration

```yaml
directory: ~/.local/share/fznote/notes
current_book: My Book
reader: bat
preview_reader: bat
editor: nvim
file_extension: md
```

Philosophy
----------

fznote doesn't reimplement anything. It shells out to fzf, git, bat, and
ripgrep, and orchestrates them into a workflow. The notes are files, the
repo is git, and the tool stays out of the way.

Errors are the underlying tools' errors. Paths are printed in full. The
tool doesn't hide what it's doing.

fznote trusts the user. It doesn't prevent unusual names, doesn't check
extensions, and doesn't try to catch mistakes you might make on purpose.
When something fails, the error tells you what happened.
