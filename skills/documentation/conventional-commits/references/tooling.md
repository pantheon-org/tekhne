# Team Tooling

## Validation — commitlint + lefthook/husky

Install and configure:

```bash
npm install --save-dev @commitlint/cli @commitlint/config-conventional
echo "export default { extends: ['@commitlint/config-conventional'] };" > commitlint.config.mjs
```

Wire up a commit-msg hook (lefthook):

```yaml
# lefthook.yml
commit-msg:
  commands:
    commitlint:
      run: npx commitlint --edit {1}
```

Run ad-hoc:

```bash
echo "feat(auth): add OAuth2 login" | npx commitlint
```

## CHANGELOG + release automation

```bash
# conventional-changelog (one-off)
npx conventional-changelog-cli -p angular -i CHANGELOG.md -s

# semantic-release (CI — reads commits since last tag, bumps version, publishes)
npx semantic-release
```

## Verify recent commit history

```bash
git log --oneline -10          # scan types at a glance
git log --format="%s" HEAD~5.. # headers only, last 5 commits
```

## References

- [commitlint Documentation](https://commitlint.js.org/)
- [conventional-changelog](https://github.com/conventional-changelog/conventional-changelog)
- [semantic-release](https://github.com/semantic-release/semantic-release)
