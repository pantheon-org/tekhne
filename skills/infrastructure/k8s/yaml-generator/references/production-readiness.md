# Production Readiness: Non-Obvious Generation Gotchas

Deep-dive reference for `k8s-yaml-generator`. Load this when generating a
Deployment or Pod spec that will run outside a scratch/dev cluster — the
defaults Kubernetes silently applies are frequently not the defaults an
author expects, and getting them wrong only surfaces under load or during a
node drain, not at `kubectl apply` time.

## `imagePullPolicy` defaults are tag-dependent, not a single default

Kubernetes does not apply one blanket default for `imagePullPolicy`. The
default is derived from the image tag:

- `image: myapp:latest` (or no tag at all) → default is `Always`. Every pod
  start re-pulls the image, even if the same tag was pulled seconds ago on
  the same node.
- `image: myapp:v1.2.3` (any non-`latest` tag) → default is `IfNotPresent`.
  A stale local image on a node is reused silently if the tag was already
  cached there, even if the registry's `v1.2.3` has since been overwritten.

Both defaults are traps in different directions: `latest` + `Always` makes
rollouts non-deterministic and slow (registry round-trip on every pod start);
a pinned tag + `IfNotPresent` can serve a stale image forever if someone
force-pushes the same tag to the registry. Always set `imagePullPolicy`
explicitly rather than relying on either inferred default, and never reuse a
tag once it has been pushed.

## Requests without limits change the pod's QoS class

The combination of `resources.requests` and `resources.limits` determines
the pod's Quality of Service class, which controls eviction order under node
memory pressure:

| requests | limits | QoS class | Eviction priority |
|---|---|---|---|
| unset | unset | `BestEffort` | Evicted first |
| set, requests < limits | set | `Burstable` | Evicted second |
| set | set, requests == limits | `Guaranteed` | Evicted last |

A manifest with `limits` but no `requests` is silently treated as
`requests == limits` (Kubernetes back-fills `requests` from `limits` when
`requests` is omitted), so it is never actually `BestEffort` — but a manifest
with `requests` and no `limits` stays `Burstable` with an unbounded ceiling,
which defeats the purpose of setting requests in the first place. For a
production workload that must survive a memory-pressure eviction event
without being picked first, set both fields, with `requests == limits`, to
land in `Guaranteed`.

## Probe threshold interaction can create restart storms

`livenessProbe` and `readinessProbe` are evaluated independently, but tuning
them identically is a common cause of cascading restarts under load: if
`initialDelaySeconds` and `periodSeconds` are equal for both probes and the
application is briefly slow to respond (a GC pause, a cold cache), the
liveness probe can fail at the same moment the readiness probe is already
pulling the pod from service — the kubelet then restarts a container that
was about to recover on its own, resetting warm caches and connections and
making the underlying slowness worse. Give the liveness probe a longer
`failureThreshold` × `periodSeconds` window than the readiness probe, so
`readinessProbe` can take the pod out of rotation without `livenessProbe`
racing to kill it.

## `terminationGracePeriodSeconds` does not wait for child processes

The default 30-second grace period sends `SIGTERM` to PID 1 in the
container and waits; it does not propagate to child processes unless PID 1
forwards the signal itself. A shell-form `CMD`/`ENTRYPOINT` (`CMD myapp
--flag`) runs the shell as PID 1 and the application as a child that never
receives `SIGTERM` directly — the shell exits, orphaning the child, which is
then hard-killed by `SIGKILL` at the end of the grace period with no chance
to flush in-flight work. Use exec-form entrypoints (`["myapp", "--flag"]`),
or a lightweight init (`tini`, `dumb-init`) as PID 1, so `SIGTERM` reaches
the actual process.
