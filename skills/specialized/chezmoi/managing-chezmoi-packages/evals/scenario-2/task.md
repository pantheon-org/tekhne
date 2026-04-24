# Distribute a Pre-Built Binary Across Mac and Linux

## Problem Description

An infrastructure engineer uses both a MacBook and a remote Linux workstation and manages their environment with chezmoi. They want to add `zellij` — a terminal workspace manager — so it's available on both machines after running `chezmoi apply`.

`zellij` distributes official pre-built binary archives on GitHub Releases at `zellij-org/zellij`. There are separate archives for macOS and Linux, and for different CPU architectures. The engineer's machines are:
- MacBook: darwin, arm64
- Linux workstation: linux, x86_64

They want the correct binary installed to `~/.local/bin/zellij` on each machine, with security verification in place. Renovate should be able to track version updates automatically.

The repo already has:
```
home/
├── .chezmoiexternals/
│   └── zsh.externals.toml.tmpl
```

## Output Specification

Produce a file `proposal.md` with:
- The full TOML content for the external definition (ready to copy-paste)
- The target file path to create in the repo
- The Renovate rule needed to automate version updates
- Commands to preview and verify before applying

The proposal should handle both macOS and Linux, and both arm64 and x86_64 architectures, using a single external definition file.

Do not apply any changes.
