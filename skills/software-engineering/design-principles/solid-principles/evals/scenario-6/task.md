# Scenario 6: Review a design for over-engineering and inheritance misuse

## User Prompt

A teammate has applied "SOLID everywhere" to a small internal tool. Review the design below and write `review.md` saying which changes to keep, which to undo, and what to do instead.

- Every one of the 22 classes has a matching `I`-prefixed interface with exactly one implementation, and none of them has ever had a second implementation or a test double.
- `PremiumUser extends AuthenticatedUser extends User extends BaseUser`, built so new perks can be added "without modifying" `User`.
- `CachedRepository.save()` throws when the cache is full, although `Repository.save()` is documented as never throwing.

## Repo state

- TypeScript project, about 3,000 lines, one team, no plug-in requirement.
- Callers of `Repository.save()` do not catch exceptions.
