# Chezmoi Templates Reference

chezmoi uses Go's `text/template` with `missingkey=error` by default (missing keys raise an error).

## Built-in Variables

```
{{ .chezmoi.hostname }}     — machine hostname
{{ .chezmoi.os }}           — "linux", "darwin", "windows"
{{ .chezmoi.arch }}         — "amd64", "arm64"
{{ .chezmoi.username }}     — current user
{{ .chezmoi.homeDir }}      — home directory path
{{ .chezmoi.sourceDir }}    — chezmoi source directory path
{{ .chezmoi.group }}        — current user's primary group
{{ .chezmoi.kernel.osRelease.id }}  — e.g. "ubuntu", "fedora" (Linux)
```

Inspect all variables: `chezmoi data`

## Conditionals

```
{{- if eq .chezmoi.os "darwin" }}
export BROWSER=open
{{- else if eq .chezmoi.os "linux" }}
export BROWSER=xdg-open
{{- end }}
```

```
{{- if eq .chezmoi.hostname "work-laptop" }}
export HTTP_PROXY=http://proxy.corp:3128
{{- end }}
```

## Custom Data

Add to `~/.config/chezmoi/chezmoi.toml`:
```toml
[data]
  email = "alice@example.com"
  workMachine = true
```

Use in templates: `{{ .email }}`, `{{ .workMachine }}`

## Password Manager Functions

```
# 1Password
{{ onepasswordRead "op://vault/item/field" }}

# Bitwarden
{{ bitwarden "item" "field" }}

# LastPass
{{ lastpass "item" "field" }}

# gopass
{{ gopass "path/to/secret" }}

# keyring (macOS Keychain / libsecret)
{{ keyring "service" "username" }}
```

## Sprig Functions

chezmoi includes the [Sprig](https://masterminds.github.io/sprig/) template library:

```
{{ "hello" | upper }}              → HELLO
{{ list "a" "b" "c" | join "," }} → a,b,c
{{ env "HOME" }}                   → /home/alice
{{ include "other-file" . }}       → render another template file
```

## Debugging Templates

```bash
# Preview rendered output without applying
chezmoi cat ~/.zshrc

# Render from stdin
echo '{{ .chezmoi.os }}' | chezmoi execute-template

# Render a specific source file
chezmoi execute-template < ~/.local/share/chezmoi/dot_zshrc.tmpl
```

## Template Options

Override `missingkey=error` in `chezmoi.toml`:
```toml
[template]
  options = ["missingkey=zero"]
```

## Whitespace Control

Use `-` to strip whitespace:
```
{{- if eq .chezmoi.os "darwin" -}}
  ...
{{- end -}}
```
