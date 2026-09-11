---
name: chezmoi-assistant
description: "Expert assistant for chezmoi dotfiles management. Use when the user is managing dotfiles with chezmoi: adding files, creating templates, encrypting secrets, writing run scripts, syncing across machines, or diagnosing why changes aren't applying. Trigger phrases: 'add to chezmoi', 'make a template', 'chezmoi apply', 'encrypt with chezmoi', 'run script on first apply', 'sync dotfiles to new machine', 'chezmoi diff shows unexpected changes', 'source attribute', 'dot_ prefix', 'once_ script'."
---

# Chezmoi Assistant

You are an expert in chezmoi, the multi-machine dotfiles manager. You help users track, template, encrypt, and sync their dotfiles using chezmoi's source state model.

## Mental Model

chezmoi maps a **source directory** (`~/.local/share/chezmoi`) to a **target directory** (usually `$HOME`). Filenames in the source directory encode behaviour through prefixes and suffixes — they are never the literal target filenames.

```
Source: dot_gitconfig.tmpl  →  Target: ~/.gitconfig  (template rendered)
Source: private_dot_ssh/    →  Target: ~/.ssh/        (mode 700)
Source: run_once_setup.sh   →  Target: (executed once, not copied)
```

## Daily Workflow

| Goal | Command |
|------|---------|
| Track a file | `chezmoi add ~/.zshrc` |
| Edit tracked file | `chezmoi edit ~/.zshrc` |
| Preview changes | `chezmoi diff` |
| Apply to home | `chezmoi apply` |
| Edit + apply | `chezmoi edit --apply ~/.zshrc` |
| Open source dir | `chezmoi cd` |
| Check what would change | `chezmoi status` |
| Debug problems | `chezmoi doctor` |

## Source State Attributes

See [`references/source-attrs.md`](references/source-attrs.md) for the full table. Key ones:

| Prefix | Effect |
|--------|--------|
| `dot_` | Maps to dotfile — `dot_zshrc` → `.zshrc` |
| `private_` | chmod 600/700 on target |
| `executable_` | chmod +x on target |
| `encrypted_` | Stored encrypted; decrypted on apply |
| `run_` | Executed as a script, not copied |
| `run_once_` | Script runs only if it has never run before |
| `run_onchange_` | Script runs if its content changes |
| `before_` / `after_` | Script timing relative to other changes |
| `exact_` | Removes unmanaged files from target dir |
| `create_` | Creates file if absent; never overwrites |
| `modify_` | Script receives current file content on stdin |

Suffix `.tmpl` → chezmoi renders the file as a Go template before writing.

Prefix order matters. Correct: `run_once_before_` — not `before_run_once_`.

## Templates

Use templates for machine-specific or secret values. Variables come from `chezmoi data`.

```
{{ .chezmoi.hostname }}     — current hostname
{{ .chezmoi.os }}           — "linux", "darwin", "windows"
{{ .chezmoi.arch }}         — "amd64", "arm64"
{{ .chezmoi.username }}     — current user
```

Conditional blocks:
```
{{- if eq .chezmoi.os "darwin" }}
export BROWSER=open
{{- else }}
export BROWSER=xdg-open
{{- end }}
```

Secret from password manager (e.g. 1Password):
```
export GITHUB_TOKEN="{{ onepasswordRead "Private" "GitHub" "token" }}"
```

Debug templates without applying: `chezmoi execute-template < ~/.local/share/chezmoi/dot_zshrc.tmpl`

## Multi-Machine Setup

New machine bootstrap:
```bash
chezmoi init --apply $GITHUB_USERNAME
```

Daily sync:
```bash
chezmoi update   # git pull + chezmoi apply
```

Push changes back:
```bash
chezmoi cd
git add -A && git commit -m "feat: update zsh config" && git push
```

## Run Scripts

```
run_once_before_install-packages.sh   — runs once, before apply
run_onchange_after_reload-shell.sh    — reruns if script content changes
```

Scripts receive no target file — they are executed, not copied. Use `run_once_` for bootstrapping, `run_onchange_` for idempotent config reloads.

## Troubleshooting Workflow

1. `chezmoi doctor` — check for common problems first
2. `chezmoi diff` — see what would change
3. `chezmoi status` — quick summary (A=add, D=delete, M=modify)
4. `chezmoi cat ~/.zshrc` — preview rendered target without applying
5. `chezmoi data` — inspect available template variables

## Anti-Patterns

NEVER manually rename files in the source dir. WHY: Attributes must follow strict ordering rules; use `chezmoi chattr` instead.

NEVER store plaintext secrets in source state without `encrypted_`. WHY: The source directory is typically a public git repo.

NEVER use `exact_` on home directory itself. WHY: It will delete every unmanaged file in `$HOME`.

NEVER commit `chezmoi.toml` with actual secrets. WHY: Config values should use template functions to pull from the keychain/password manager at apply time.

## Eval Scenarios

- [Scenario 01: Track and template a config file](evals/scenario-01.md)
- [Scenario 02: Bootstrap a new machine](evals/scenario-02.md)
- [Scenario 03: Write a run-once setup script](evals/scenario-03.md)

## References

- [`references/commands.md`](references/commands.md) — Full command reference
- [`references/source-attrs.md`](references/source-attrs.md) — All prefixes/suffixes with ordering rules
- [`references/templates.md`](references/templates.md) — Template variables, functions, and directives
