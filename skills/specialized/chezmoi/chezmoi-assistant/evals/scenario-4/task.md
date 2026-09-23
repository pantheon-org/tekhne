# Scenario 4: Add AWS credentials to chezmoi safely

## User Prompt

I want chezmoi to manage my `~/.aws/credentials` file so it syncs to my other machines. The repo I push my dotfiles to is on GitHub.

## Expected Behavior

1. Warn that `~/.aws/credentials` contains plaintext secrets and must not be tracked as a plain or `private_` file, since `private_` only sets permissions on the target machine and does nothing to protect the plaintext in git history
2. Recommend the `encrypted_` prefix (e.g. `encrypted_private_dot_aws/credentials.asc`) combined with chezmoi's configured encryption backend (age or gpg)
3. Explain that `chezmoi add --encrypt ~/.aws/credentials` handles the encryption step automatically
4. Note that anyone who can read the git history — including a repo that later becomes public or gets forked — could otherwise recover the plaintext forever, even after a later commit removes it
5. Confirm the file decrypts transparently on `chezmoi apply` on a machine with the right key configured

## Success Criteria

- Explicitly flags that `private_` alone is insufficient for a secret file
- Uses `encrypted_` prefix or `chezmoi add --encrypt`
- Explains the plaintext-in-git-history consequence of skipping encryption
- Does not suggest committing the file as a normal or `private_`-only file

## Failure Conditions

- Tracks the credentials file as a plain or `private_`-only file without flagging the risk
- Fails to mention that git history retains plaintext even after later removal
- Confuses `private_` (permissions) with `encrypted_` (encryption at rest)
