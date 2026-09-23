# Scenario 5: Rename a tracked file and scope `exact_` correctly

## User Prompt

I tracked `~/.config/nvim` with chezmoi using `exact_` so leftover plugin files get cleaned up, but now I want to rename the source entry and also make sure I haven't accidentally told chezmoi to `exact_` my whole home directory.

## Expected Behavior

1. Warn against manually `mv`-ing files inside the source directory, since chezmoi encodes target path, template status, and encryption entirely in the filename and a plain rename can desync source state from what's on disk
2. Recommend `chezmoi chattr` (or re-deriving the source name via `chezmoi re-add`) instead of a raw filesystem rename
3. Confirm `exact_` is safe when scoped to a specific subtree like `exact_dot_config/nvim/`, because it only removes unmanaged files under that one directory
4. Warn explicitly that applying `exact_` at the top level (equivalent to `$HOME` itself) would delete every unmanaged file in the user's home directory with no extra confirmation
5. Show how to check current attributes with `chezmoi cat` or `chezmoi managed` before making the change

## Success Criteria

- Explicitly warns against manual `mv`/rename inside the source directory and names `chezmoi chattr` as the safe alternative
- Confirms the nvim-scoped `exact_` usage is fine
- Explicitly warns that a top-level (`$HOME`-scoped) `exact_` would delete unmanaged files with no extra prompt
- Distinguishes "scoped safely" from "applied at the top level" rather than treating all `exact_` usage the same

## Failure Conditions

- Approves a raw `mv`/rename of a source-directory file without warning about desync
- Fails to flag the danger of `exact_` at the home-directory root
- Treats `exact_dot_config/nvim/` and a top-level `exact_` as equally risky (over-warns) or equally safe (under-warns)
