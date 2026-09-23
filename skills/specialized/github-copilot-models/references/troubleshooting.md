# Troubleshooting GitHub Copilot Model Access

## "Provider not found"

The provider name OpenCode expects doesn't match what's configured. Target it explicitly:

```bash
opencode models github-copilot
```

If this still fails, the GitHub Copilot provider itself may not be configured in this OpenCode installation — check `opencode auth list` for `github-copilot` in the output before assuming a model-specific problem.

## 401 / "Invalid token"

The cached auth token has expired or been revoked. Re-authenticate:

```bash
opencode auth add github-copilot
opencode auth list   # confirm the new token is present
```

A 401 from the models endpoint specifically (rather than from every OpenCode command) usually means the token is stale rather than missing — GitHub Copilot tokens have a shorter lifetime than the OpenCode session itself.

## Models not showing up that you expect

OpenCode's bundled registry is a release-time snapshot (see the skill's Mindset section) — it does not reflect live account changes. Always cross-check with a direct query before concluding a model genuinely isn't available:

```bash
scripts/fetch-models.sh --json | jq '.data[] | select(.id == "the-model-you-expect")'
```

An empty result here (rather than just "not shown by OpenCode") means the account genuinely does not have that model, which is a different problem — subscription tier, region, or org policy — than a stale local registry.

## Model unavailable at runtime despite being listed

Check both gating fields, since either alone can cause a runtime rejection even though the model appears in a plain listing:

```bash
scripts/fetch-models.sh --json | jq '.data[] | select(.id == "the-model-in-question") | {policy: .policy.state}'
```

- `policy.state != "enabled"` (i.e. `"disabled"` or restricted) — the model is not usable regardless of what the listing shows; pick a different `"enabled"` model.
- `preview` flag set — availability may depend on subscription tier and can be withdrawn without notice; treat as unsuitable for anything unattended (see the "NEVER treat a `preview` model as production-safe" anti-pattern in `SKILL.md`).

## Diagnosing a sudden "it worked yesterday" failure

This is almost always a `policy.state` change on GitHub's side, not a local configuration regression. The fastest diagnosis:

1. Re-query the model's current `policy.state` directly — don't trust the last-known-good assumption.
2. If `disabled` or newly restricted, that confirms the root cause: a provider-side policy change, not a bug in the pipeline.
3. Discover a replacement with the same category tags (`powerful`/`versatile`/`fast`) and an `"enabled"` state, then validate it with a single test call before wiring it in as the new default.
