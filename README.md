fznote - Fuzzy Note Manager
---------------------------

fznote is a lightweight, terminal-based note manager that combines the power
of fzf with the simplicity of plain text files.

## Features

    🎯 Fuzzy search - Find any note instantly with fzf

    📝 Plain text - Your notes are just files. Use any editor, any format

    🔒 Git-backed - Version control built in. Commit, push, never lose work

    🎨 Preview and printing with configurable reader

    🔌 Terminal-native - Works in your workflow, not against it

## Installation
git clone https://github.com/Njoter/fznote
cd fznote
cargo install --path .

## Requirements

*   fzf - The heart of fznote. Required for fuzzy finding.
*   some kind of reader. I recommend bat.
*   ripgrep - Optional for content search

## Basic commands
fznote

fznote read

fznote edit

fznote add mynote.md

fznote remove

fznote search "rust"

fznote push

## status
fznote is in early development. Things may be changed.
