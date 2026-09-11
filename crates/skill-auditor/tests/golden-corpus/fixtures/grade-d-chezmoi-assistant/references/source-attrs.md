# Source State Attributes

## File Prefixes (in order)

| Prefix | Effect |
|--------|--------|
| `after_` | Run script after updating the destination |
| `before_` | Run script before updating the destination |
| `create_` | Ensure file exists; create with contents if absent (never overwrites) |
| `dot_` | Rename to use a leading dot — `dot_foo` → `.foo` |
| `empty_` | Ensure file exists even if empty (by default empty files are removed) |
| `encrypted_` | Encrypt file in source state; decrypt on apply |
| `exact_` | Remove anything in the target dir not managed by chezmoi |
| `executable_` | Add executable permissions to target |
| `external_` | Ignore attributes in child entries |
| `literal_` | Stop parsing further prefix attributes |
| `modify_` | Treat contents as a script that modifies an existing file (receives current file on stdin) |
| `once_` | Only run the script if its contents have never run before |
| `onchange_` | Only run if content has changed since last run |
| `private_` | Remove all group and world permissions (chmod 600/700) |
| `readonly_` | Remove all write permissions |
| `remove_` | Remove the file/symlink/empty-dir if it exists |
| `run_` | Treat contents as a script to run (not copied to target) |
| `symlink_` | Create a symlink instead of a regular file |

## File Suffixes

| Suffix | Effect |
|--------|--------|
| `.tmpl` | Render contents as a Go template before applying |
| `.literal` | Stop parsing further suffix attributes |

## Ordering Rules by Target Type

| Target type | Allowed prefixes (in order) | Allowed suffixes |
|-------------|----------------------------|-----------------|
| Directory | `remove_`, `external_`, `exact_`, `private_`, `readonly_`, `dot_` | none |
| Regular file | `encrypted_`, `private_`, `readonly_`, `empty_`, `executable_`, `dot_` | `.tmpl` |
| Create file | `create_`, `encrypted_`, `private_`, `readonly_`, `empty_`, `executable_`, `dot_` | `.tmpl` |
| Modify file | `modify_`, `encrypted_`, `private_`, `readonly_`, `executable_`, `dot_` | `.tmpl` |
| Remove file | `remove_`, `dot_` | none |
| Script | `run_`, then `once_` or `onchange_`, then `before_` or `after_` | `.tmpl` |
| Symlink | `symlink_`, `dot_` | `.tmpl` |

## Common Examples

```
dot_zshrc                          → ~/.zshrc
dot_zshrc.tmpl                     → ~/.zshrc  (template rendered)
private_dot_ssh/                   → ~/.ssh/   (mode 700)
executable_dot_local/bin/my-script → ~/.local/bin/my-script  (chmod +x)
encrypted_private_dot_netrc        → ~/.netrc  (encrypted + chmod 600)
run_once_before_install-homebrew.sh → executed once before apply
run_onchange_after_reload-launchd.sh → executed when script changes
exact_dot_config/fish/             → ~/.config/fish/ (removes unmanaged files)
create_dot_hushlogin               → ~/.hushlogin  (created if absent, never overwritten)
```

## Changing Attributes

Use `chezmoi chattr` — do not rename files manually:
```bash
chezmoi chattr +template ~/.zshrc       # add .tmpl suffix
chezmoi chattr +encrypted ~/.netrc      # add encrypted_ prefix
chezmoi chattr +executable ~/.local/bin/myscript
chezmoi chattr -private ~/.config/some/file
```
