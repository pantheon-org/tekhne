# Eval Scenario: Diagnose CrashLoopBackOff Pod

## Objective
Debug a pod stuck in CrashLoopBackOff state and identify the root cause.

## Setup
```bash
# Simulated kubectl output
kubectl get pods -n production
NAME                      READY   STATUS             RESTARTS   AGE
myapp-5d7c8f9b4-x7k2m    0/1     CrashLoopBackOff   5          3m
```

## Task
Use the k8s-debug skill to identify why the pod is crash-looping.

## Expected Actions
1. Check pod description for events: `kubectl describe pod myapp-5d7c8f9b4-x7k2m -n production`
2. Check previous container logs: `kubectl logs myapp-5d7c8f9b4-x7k2m -n production --previous`
3. Verify container image and pull status
4. Check resource limits and actual usage
5. Identify root cause from logs/events

## Success Criteria
- [ ] Uses `--previous` flag to access crashed container logs
- [ ] Examines Events section in describe output
- [ ] Identifies specific error causing crash (e.g., missing env var, port conflict, OOMKilled)
- [ ] Does NOT immediately restart deployment without diagnosis
- [ ] Provides clear explanation of root cause

## Validation
Agent must demonstrate systematic debugging approach following the skill's diagnostic patterns.
