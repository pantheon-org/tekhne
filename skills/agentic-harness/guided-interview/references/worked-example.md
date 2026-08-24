# Worked Example — End-to-End Interview

A fully annotated interview about choosing a caching strategy, showing branching and the
recap/confirm step.

## Turn 1 — User request

> "I need to pick a caching strategy for our API. Interview me about it."

Scope is already clear from the request — no throat-clearing question needed. Go straight to
question 1.

## Turn 2 — Agent asks Q1

> Where should the cache live?
>
> 1. **In-process** — fastest, but lost on every restart and not shared across instances
> 2. **Shared cache (Redis/Memcached)** — survives restarts, shared across instances, adds a
>    network hop
> 3. **CDN/edge cache** — best for public, cacheable responses; not suited to per-user data
> 4. *(free text)* — describe something else

## Turn 3 — User answers

> "Shared cache — we have multiple instances behind a load balancer."

**Adaptation:** the answer rules out in-process caching entirely and confirms the system is
multi-instance. The next question should build on "shared cache," not repeat a generic list.

## Turn 4 — Agent asks Q2 (adapted)

> For the shared cache, what's the main invalidation trigger?
>
> 1. **Time-based (TTL)** — simplest, accepts some staleness
> 2. **Event-based** — invalidate on write, freshest but more moving parts
> 3. **Manual/admin-triggered** — rare updates, low automation need
> 4. *(free text)*

## Turn 5 — User answers

> "Event-based — writes are infrequent but must be reflected immediately."

**Sufficiency check:** location (shared cache) and invalidation strategy (event-based) are enough
to write a recommendation. Stop asking.

## Turn 6 — Recap and confirm

> Here's what I have:
>
> - Cache location: shared cache (Redis/Memcached), driven by multiple instances behind a load
>   balancer
> - Invalidation: event-based, triggered on write, because staleness is not acceptable
>
> Did I get that right? Anything you'd change before I write this up?

## Turn 7 — User confirms

> "Yes, that's right."

## Turn 8 — Output

Because this interview was a standalone decision (not feeding a plan or ticket the user already
asked for), the output stays in chat: a short synthesized recommendation naming Redis with
write-triggered invalidation, plus the one tradeoff worth flagging (added infrastructure to
operate). No file is produced unless the user then asks for one.
