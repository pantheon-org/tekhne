# Eval Scenario: Service Not Routing to Pods

## Objective
Diagnose why a service is not routing traffic to healthy pods.

## Setup
```bash
# Service exists and pods are running
kubectl get svc myapp-service -n production
NAME             TYPE        CLUSTER-IP     EXTERNAL-IP   PORT(S)   AGE
myapp-service    ClusterIP   10.96.45.123   <none>        80/TCP    1h

kubectl get pods -n production -l app=myapp
NAME                    READY   STATUS    RESTARTS   AGE
myapp-5d7c8f9b4-abc12   1/1     Running   0          5m
myapp-5d7c8f9b4-def34   1/1     Running   0          5m
```

## Task
Identify why the service endpoints are empty and traffic is not routing.

## Expected Actions
1. Check service selector: `kubectl get svc myapp-service -o jsonpath='{.spec.selector}'`
2. Check pod labels: `kubectl get pods -l app=myapp --show-labels`
3. Verify endpoints: `kubectl get endpoints myapp-service`
4. Identify label mismatch between service selector and pod labels

## Success Criteria
- [ ] Checks service selector explicitly
- [ ] Verifies pod labels match selector
- [ ] Uses `kubectl get endpoints` to confirm routing
- [ ] Identifies label mismatch as root cause
- [ ] Suggests correct fix (update selector or labels)

## Validation
Agent must follow the "ALWAYS: Check Labels and Selectors First" anti-pattern guidance.
